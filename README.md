# NEAR Linkdrop NFT Proxy Contract

This allows users to create linkdrops for much cheaper and they can preload accounts with NFTs than if they were to interact with the original [linkdrop](https://github.com/mattlockyer/linkdrop-proxy) contract directly. The current cost is ~0.023 $NEAR (which can be refunded) for a basic account. If you deposit more $NEAR, the claimed account will start with more $NEAR. More information is outlined in the `linkdrop.rs` file.

This is in contrast with the regular [linkdrop](https://github.com/mattlockyer/linkdrop-proxy) contract which charges ~1 $NEAR per account.

The NFTs are currently hardcoded to be Go Team NFTs and are minted on this contract itself. Future iterations will use cross-contract calls and the NFT and linkdrop proxy contract will be seperated. In addition, GAS fees, storage fees, and absolute minimum account storage fees will be profiled to reduce overall costs.

This contract is built using a modified version of Matt Lockyer's proxy [contract]() and we're working closely together to expand and build more contracts that are optimized and can do more.

## Prerequisites

* [Rust Toolchain](https://docs.near.org/docs/develop/contracts/rust/intro#installing-the-rust-toolchain)
* [NEAR-CLI](https://docs.near.org/docs/tools/near-cli#setup)
* [yarn](https://classic.yarnpkg.com/en/docs/install#mac-stable)

# Quick-Start 

In this repo, there is a `quickstart.js` script that will create a linkdrop. There is also the actual core contract that can be built and deployed. To get started, install all the necessary packages:


```
npm i
```

next, build and deploy contract: 

```
yarn build && near dev-deploy out/main.wasm
```

next, go into the `quickstart.js` file and replace the following lines with the contract ID you deployed to and the funding account that you want to use.

```
let LINKDROP_PROXY_CONTRACT_ID = "dev-1652290060466-30433495201402";
let FUNDING_ACCOUNT_ID = "benjiman.testnet";
```

to create a linkdrop, run the quickstart script but make sure you have the credentials to the linkdrop proxy contract and funding account ID stored in your local file system (`~/.near-credentials/testnet`).

```
node quickstart.js
```

This will initialize the contract with default metadata and create a linkdrop with an attached 0.9778 $NEAR (1 $NEAR - 0.2282 $NEAR). If you want to change this value, you can edit the `parseNearAmount('1')` value in line 69 of the `quickstart.js` file.

To access the linkdrop, the console will log the link you need to navigate to: 
<img width="1723" alt="image" src="https://user-images.githubusercontent.com/57506486/167914646-a62b1b70-1999-4619-9727-61bbb31a4483.png">

```
https://wallet.testnet.near.org/linkdrop/dev-1652291366144-14872936582999/4RL21tCWig5ZXiqcFaBgSGUNLkk41UJjhWz62wyNrtHC1rnYxJx6XjQpV1bkJ4ttUeUawdm8wL8xRScH1MqJj8YU
```

After clicking the link and either claiming with an existing account or creating a new account, navigate to the collectibles tab and you should have your NFT: 

<img width="660" alt="image" src="https://user-images.githubusercontent.com/57506486/167914914-cccd65e8-5a56-486e-8811-d5d4a32ffe14.png">


