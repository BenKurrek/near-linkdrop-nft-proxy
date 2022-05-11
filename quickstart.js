const { connect, KeyPair, keyStores, utils } = require("near-api-js");
const { parseNearAmount } = require("near-api-js/lib/utils/format");
const path = require("path");
const homedir = require("os").homedir();

let LINKDROP_PROXY_CONTRACT_ID = "dev-1652290060466-30433495201402";
let FUNDING_ACCOUNT_ID = "benjiman.testnet";
let NETWORK_ID = "testnet";
let near;
let config;
let keyStore;

// set up near
const initiateNear = async () => {
	const CREDENTIALS_DIR = ".near-credentials";

	const credentialsPath = (await path).join(homedir, CREDENTIALS_DIR);
	(await path).join;
	keyStore = new keyStores.UnencryptedFileSystemKeyStore(credentialsPath);

	config = {
		networkId: NETWORK_ID,
		keyStore,
		nodeUrl: "https://rpc.testnet.near.org",
		walletUrl: "https://wallet.testnet.near.org",
		helperUrl: "https://helper.testnet.near.org",
		explorerUrl: "https://explorer.testnet.near.org",
	};

	near = await connect(config);
};

async function start() {
    //deployed linkdrop proxy contract
	const contractId = LINKDROP_PROXY_CONTRACT_ID;
	await initiateNear();

	const contractAccount = await near.account(contractId);

	const sendingAccount = await near.account(FUNDING_ACCOUNT_ID);

	console.log("initializing contract");
	try {
		await contractAccount.functionCall(
			contractId, 
			'new_default_meta', 
			{
				owner_id: contractId,
			}, 
			"300000000000000", 
		);
	} catch(e) {
		console.log('error initializing contract: ', e);
	}
	
	let keyPair = await KeyPair.fromRandom('ed25519'); 
	let pubKey = keyPair.publicKey.toString();
	console.log('pubKey: ', pubKey);

	console.log("sending funds as FUNDING_ACCOUNT_ID");
	try {
		await sendingAccount.functionCall(
			contractId, 
			'send', 
			{
				public_key: pubKey,
			}, 
			"300000000000000", 
			parseNearAmount('1')
		);
	} catch(e) {
		console.log('error initializing contract: ', e);
	}

	console.log(`https://wallet.testnet.near.org/linkdrop/${contractId}/${keyPair.secretKey}`);
}

start();