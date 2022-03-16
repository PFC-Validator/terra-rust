# Changes
## 1.0
### 1.0.12 - 16-Mar-2002
* expand block now can expand from a list of variables 'V:xxx' passed into command. optionally ignoring the failure (so we can multipass)
### 1.0.11 - 15-Mar-2022
* seed_from_args / wallet_opt_from_args functions
* expand block now expands A:key and O:key to account/operator address for key in wallet
### 1.0.9 - 15-Mar-2022
* ###SENDER### will only be replaced if sender is specified. Tighten Regex
### 1.0.7 - 13-Mar-2022
* new functions 'get_json_block_expanded/expand_block' replaces ###E:xxx### with environment variable xxx &amp; ###SENDER### with sender
### 1.0.6 - 25-Feb-2022
* additional helpers for reading private key
### 1.0.4 - 21-Feb-2022
* remove depreciation warnings
### 1.0.3 - 20-Feb-2022
* get_json_block - reads stdin/file/or json passed as an argument
### 1.0.2 - 16-Feb-2022
* error returns
### 1.0.1
* add more helpers
### 1.0.0 - 14-Feb-2022
* moved to own library to help other utilities