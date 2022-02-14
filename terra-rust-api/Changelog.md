# Changes

## 1.2
### 1.2.4 - Devel
### 1.2.3 - 14-Feb-22
* PR#8 : use default for sequence deserialize thanks @patternmachine!
### 1.2.2 - 6-Feb-22
* Block Result txs_result can be null
### 1.2.1 - 31-Jan-22
* TX block doesn't require contract/execute_msg parts when it is returned (backwards incompatible)
* MsgMigrateContract message added
## 1.1
### 1.1.8 - 17-Dec-21
* extra fields in SyncInfo area in RPC
* added helper function to EventAttributes
* added serialization to some structures 
### 1.1.3 - 16-Dec-21
* RPC Block/Block_Result api
* New deserializer for base64 encoded strings, base64_encoded_format & base64_opt_encoded_format
### 1.1.2 - 14-Dec-21
* parse IBC coins
* tx.get_txs_in_block() function
### 1.1.1 - 22-Nov-21
* PR #6 - Thanks @Pronvis
* PR #6 - return Result in most cases
* PR #6 - better usage of async/await
* PR #6 - lifetime fixes
## 1.0
### 1.0.12 - 3-Nov-21
* add 'tx' component of TxResult
### 1.0.11 - 28-Oct-21
* tx attribute finder helper function
### 1.0.10 - 27-Oct-21
* tx.timestamp now visible
* tx.get_events() helper function
### 1.0.9 - 26-Oct-21
* tx.logs can be null
### 1.0.8 - 9-Oct-21
* WASM Exec contract working again - (Thx @sambarboza)
* HTTP responses now TerraLCDResponse error types
* TX response work.
* new example 'set_code' .. instantiate code TBD
* Error's returned are now TerraRustAPIError
### 1.0.7 - 2-Oct-21
* fix up some documentation links (Thx @PSchlutermann)
### 1.0.6 - 1-Oct-21
* fix up estimation/can send basic txs
### 1.0.1 - 30-Sep-21
* Switch to Col-5
## 0.3
### 0.3.8 - (never released)
* parse tendermint hex-addresses and display tendermint_address()
* API function PublicKey::from_tendermint_address() 
### 0.3.7 - 17-Sep-21
* new API(s) tendermint.validatorsets_full(_at_height) - fetch ALL the validator, bypassing the hardcoded limit 
### 0.3.6 - 16-Sep-21
* Add 'Clone' to tendermint/validator
### 0.3.4 - 13-Sep-21
* TX Block Attributes can have 'value' as null
* Add 'Clone' to various structures
### 0.3.3 - 27-Aug-21
* tendermint_types:Block structure can have 'data' as null.
### 0.3.2 - 24-Aug-21
* Switched LCD query responses to generic type (API Change)
* RPC endpoint API started
* FCD endpoint API started
* can now fetch gas prices from FCD
* add Eq/ToString to NodeIpPort
## 0.2
### 0.2.9 - 18-Aug-21
* Add slashing/Unjail
* Add market/Swap 
* Add "sweep" function to mass swap tokens above a certain threshold
### 0.2.8 - not usable
### 0.2.7 - 12-Aug-21
* Add timezone support (used on tendermint dates in addressbook)
* Add functions to fetch address book
* Add functions to display delegations/undelegations
* Helper functions submit_transaction_{async|sync}
* Add messages for delegator rewards, and around staking/unstaking

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
