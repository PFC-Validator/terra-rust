# Changes

## 0.2
### 0.2.7 - Devel
### 0.2.6 - 29-July-21
* PR #2 linux build by [@snoberg](https://github.com/snoyberg)
* PR #3 ability to use rusttls [@snoberg](https://github.com/snoyberg)
* clippy warnings
### 0.2.5 - 8-July-21 
* added some more documentation
* [BUG] Tendermint/blocks - signatures can be null
* added tendermint /validatorsets API call
* added terra_i64_format
### 0.2.4 - 8-July-21
* Switch to thiserror (api) & anyhow (command) error handling
* EditValidator Message working
### 0.2.3 -  1-Jun-21
* contract execution/queries
* terra_opt_u64_format / Option<u64> json formatting

### 0.2.1 - 24-May-21
* Restructured messages API to hopefully make it easier
* Added basic support for ED25519 keys (which are used in tendermint consensus)
* Wallet functionality taken out to separate library
* Oracle & Staking messages added. still in beta
