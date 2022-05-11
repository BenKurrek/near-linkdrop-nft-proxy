use crate::*;
use near_sdk::{ext_contract, Gas, PromiseResult};

/* 
    0.001 N 
    minimum amount of storage required to cover:
    - storing access key on the contract
    - storing pub key and deposit

    Some of this can be refunded once the account is claimed.
*/                               
const MIN_STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;

/* 
    0.02 N 
    allowance for the access key to cover GAS fees when the account is claimed. This amount will never
    be refunded as it's burnt.

    If this is lower, it will throw the following error:
    Access Key {account_id}:{public_key} does not have enough balance 0.01 for transaction costing 0.018742491841859367297184
*/                               
const MIN_ACCESS_KEY_ALLOWANCE: u128 = 20_000_000_000_000_000_000_000;

/* 
    minimum amount of NEAR that a new account must have when created (0.00182 $NEAR)
    If this is 0, it will throw the following error:
    "LackBalanceForState": {
        "account_id": "zbenji3.testnet",
        "amount": "1820000000000000000000" 
    }
*/ 
const NEW_ACCOUNT_ABSOLUTE_MINIMUM: u128 = 1_820_000_000_000_000_000_000;


const ON_CREATE_ACCOUNT_GAS: Gas = Gas(40_000_000_000_000);
const ON_CALLBACK_GAS: Gas = Gas(20_000_000_000_000);
/// Indicates there are no deposit for a callback for better readability.
const NO_DEPOSIT: u128 = 0;

/// external and self callbacks
#[ext_contract(ext_linkdrop)]
trait ExtLinkdrop {
    fn create_account(&mut self, new_account_id: AccountId, new_public_key: PublicKey) -> Promise;
}
#[ext_contract(ext_self)]
trait ExtLinkdrop {
    fn on_account_created(&mut self, pk: PublicKey, new_account_id: AccountId, linkdrop_amount: Balance) -> bool;
}

#[near_bindgen]
impl Contract {
    //set the desired linkdrop contract to interact with
	pub fn set_contract(&mut self, linkdrop_contract: AccountId) {
		assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "predecessor != current"
        );
		self.linkdrop_contract = linkdrop_contract;
	}

    /*
        user has created a keypair and passes in the public key and attaches some deposit.
        this will store the amount and allow that key to call claim and create_account_and_claim
        on this contract.
    */
    #[payable]
    pub fn send(&mut self, public_key: PublicKey) {
        //check if the public key exists yet.
        let pk_exists = self.accounts.contains_key(&public_key); 

        /* 
            Minimum required initial deposit consists of the storage cost, GAS fees, and the absolute minimum required to fund an account.
            If there is any excess, it will be automatically sent to the account claiming the linkdrop on top of the absolute minimum.
            The contract can refund the minimum storage cost at a later date since that's freed up once the linkdrop is claimed.
        */ 
        let mut min_required_initial_deposit: u128 = MIN_STORAGE_COST + MIN_ACCESS_KEY_ALLOWANCE + NEW_ACCOUNT_ABSOLUTE_MINIMUM;
        
        /*
            if the PK already exists, there's no need to pay for storage or the allowance, or the new account absolute minimum.
            Any attached deposit in this case is automatically added to the linkdrop.
        */ 
        if pk_exists {
            min_required_initial_deposit = 1
        }
        
        //ensure the user has attached enough $NEAR 
        assert!(
            env::attached_deposit() >= min_required_initial_deposit,
            "Deposit < MIN_REQUIRED_INITIAL_DEPOSIT"
        );

        /*
            default the amount to deposit to be equal to the attached deposit minus the GAS fees minus storage cost.
            If the user attached the exact minimum initial deposit, this value will be equal to NEW_ACCOUNT_ABSOLUTE_MINIMUM.
            Anything extra will be added to the amount to deposit.
        */
        let mut amount_to_deposit = env::attached_deposit() - MIN_ACCESS_KEY_ALLOWANCE - MIN_STORAGE_COST;

        
        //if the public key already exists, any attached deposit is automatically added to the linkdrop.
        if pk_exists {
            amount_to_deposit = env::attached_deposit()
        }

        //add the amount to deposit for the given public key.
        self.accounts.insert(
            &public_key,
            &(self.accounts.get(&public_key).unwrap_or(0) + amount_to_deposit),
        );

        /*
            if the public key doesn't exist, we'll add it as an access key to the contract 
            which can only call claim and create_account_and_claim on this contract
        */
        if !pk_exists {
            Promise::new(env::current_account_id()).add_access_key(
                public_key,
                MIN_ACCESS_KEY_ALLOWANCE,
                env::current_account_id(),
                "claim,create_account_and_claim".to_string(),
            );
        }        
    }

	/*
        internal method for claiming tokens and deleting the used key. 
        this is called within claim and create_account_and_claim which is called
        in the wallet by using the function call access key and signing using the 
        contract account.
    */
	fn process_claim(&mut self) -> Balance {
        //ensure that only the current account is the predecessor (by using the function call access key)
		assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "predecessor != current"
        );

        //get the linkdrop amount for the public key (and remove it from the map)
        let amount = self
            .accounts
            .remove(&env::signer_account_pk())
            .expect("Missing public key");

        //delete the public key from the contract. This will free up the storage on this contract. 
		Promise::new(env::current_account_id()).delete_key(env::signer_account_pk());

		amount
	}

    /*
        claim the tokens with an existing account. If the account doesn't exist, the contract will keep the funds.
        This will also mint an NFT to the claiming user.
    */
    pub fn claim(&mut self, account_id: AccountId) -> Promise {
        //get the linkdrop amount and delete the access key.
        let amount = self.process_claim();
		
        //mint an NFT to the account ID
        self.nft_mint(
            format!("token_{}", self.token_nonce),
            TokenMetadata {
                title: Some("Linkdrop GoTeam Token!".to_string()),
                description: Some("This is a test token for linkdrop contracts".to_string()),
                media: Some("https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif".to_string()),
                media_hash: None,
                copies: None,
                issued_at: None,
                expires_at: None,
                starts_at: None,
                updated_at: None,
                extra: None,
                reference: None,
                reference_hash: None
            },
            account_id.clone(),
            None,
        );

        //send the account the linkdrop amount.
        Promise::new(account_id).transfer(amount)

        //TODO: transfer the released storage to the original sender
    }

    /*
        claim tokens with an account that doesn't exist. This will create the account and mint the NFT.
    */
    pub fn create_account_and_claim(
        &mut self,
        new_account_id: AccountId,
        new_public_key: PublicKey,
    ) -> Promise {
        //get the linkdrop amount and delete the access key
        let amount = self.process_claim();

        //cross contract call to the linkdrop contract to create the account
        ext_linkdrop::create_account(
            new_account_id.clone(),
            new_public_key,
            self.linkdrop_contract.clone(),
            amount,
            ON_CREATE_ACCOUNT_GAS,
        //ensure everything went well and mint the NFT. Revert logic if things went poorly (add access key back and amount)
        ).then(ext_self::on_account_created(
			env::signer_account_pk(),
            new_account_id,
            amount,
			env::current_account_id(),
			NO_DEPOSIT,
			ON_CALLBACK_GAS,
		))
    }

	/// self callback checks if account was created successfully or not
    pub fn on_account_created(&mut self, pk: PublicKey, new_account_id: AccountId, linkdrop_amount: Balance) -> bool {
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "predecessor != current"
        );
		assert_eq!(env::promise_results_count(), 1, "no promise result");
        let creation_succeeded = matches!(env::promise_result(0), PromiseResult::Successful(_));
        if !creation_succeeded {
            //add the linkdrp amount back for the public key
            self.accounts.insert(
                &pk,
                &(linkdrop_amount),
            );

			// put access key back (was deleted before calling linkdrop contract)
            Promise::new(env::current_account_id()).add_access_key(
				pk,
				MIN_ACCESS_KEY_ALLOWANCE,
				env::current_account_id(),
				"claim,create_account_and_claim".to_string(),
			);
        } else {
            //mint the NFT
            self.nft_mint(
                format!("token_{}", self.token_nonce),
                TokenMetadata {
                    title: Some("Linkdrop GoTeam Token!".to_string()),
                    description: Some("This is a test token for linkdrop contracts".to_string()),
                    media: Some("https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif".to_string()),
                    media_hash: None,
                    copies: None,
                    issued_at: None,
                    expires_at: None,
                    starts_at: None,
                    updated_at: None,
                    extra: None,
                    reference: None,
                    reference_hash: None
                },
                new_account_id,
                None,
            );

            //TODO: transfer the released storage to the original sender
        }
        creation_succeeded
    }

    /// Returns the balance associated with given key.
    pub fn get_key_balance(&self, key: PublicKey) -> U128 {
        self.accounts
            .get(&key)
            .expect("Key missing")
            .into()
    }
}
