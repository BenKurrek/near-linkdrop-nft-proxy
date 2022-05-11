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
