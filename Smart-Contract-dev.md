# Terra Rust - How to use in smart contract development
get the package ;-)

`cargo install terra-rust`

## Setup your repo
1. In your contract directory create a '.env' file similar to the [.env.default](.env.default)
2. install 'run-script' ```cargo install run-script```

3. create a .cargo file with the following
```toml
[alias]
wasm = "build --release --target wasm32-unknown-unknown"
unit-test = "test --lib"
schema = "run --example schema"
optimize = "run-script optimize"
optimize-w32 = "run-script optimize-w32"
store = "run-script store"
instantiate = "run-script instantiate"
migrate = "run-script migrate"
```
4. in your `Cargo.toml` file add
```toml
[package.metadata.scripts]
optimize = """docker run --rm -v "$(pwd)":/code \
          --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
            --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
              cosmwasm/rust-optimizer:0.12.3"""
optimize-w32 = """docker run --rm -v c:\\<your source directory>:/code  \
            --mount type=volume,source=project_name_cache,target=/code/target \
            --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
            cosmwasm/rust-optimizer:0.12.3"""
store = """terra-rust code store test-owner ..\\..\\artifacts\\xxx.wasm"""
instantiate = """terra-rust code instantiate test-owner ..\\..\\artifacts\\xxx.wasm .\\pool_init.json --admin same"""
migrate = """terra-rust code migrate test-owner ..\\..\\artifacts\\xxx.wasm """

```
In the above example, you will need to hard code your path in optimize-w32, and ideally make the project_name_cache unique for your contract
also I have used the 'test-owner' account as the 'owner/admin' of the contract. ('same' just means use the same account as the owner)

The pool_init.json is the 'init' json file ... 
The code will replace `##SENDER##` `##CODE_ID##` `###NEW_CODE_ID###` and `##ADMIN##` fields with their respective values

The migrate command takes a json file as well, but defaults to '{}'

The first time you instantiate your code, you will be given a contract. put that in your [.env](.env.default) as **TERRARUST_CONTRACT** file so migrates are easier.
**TERRARUST_GAS_DENOM**=uusd might be useful as well.


## Setup your environment

### create a test wallet to store all your test keys in.
   1. `terra-rust create testing`
   2. `terra-rust --wallet testing keys new xxx` (if you are in the .env directory the wallet can be filled in from the .env file)
   3. probably go to [faucet](https://faucet.terra.money/) and put some $ into them?

# tools
* [terra-exec](src/bin/terra_exec.rs) is a command line tool to make contract exec really simple. 
* [terra-query](src/bin/terra_query.rs) is a command line tool to make contract query really simple. 


## terra_exec
set **TERRARUST_CONTRACT** in your environment, usually via placing a line in your .env file.

to query (notice the escaped double quotes)
```

C:> terra-query --contract terra13fs83g5atgjwuh7c5ydzh6n7gecel6xyhhy2t5 '{\"token_info\":{}}'

```

to exec a smart contract method add the 'sender' 
```
C:> terra-exec --sender test-owner --contract terra13fs83g5atgjwuh7c5ydzh6n7gecel6xyhhy2t5 '{\"set_minter\":\"terra1fmqjnum0ftyuc72mapg3wqpf9m7jwdtnkp7960\"}'
 
```
# How can you help?
If you think this was useful, feel free to delegate to the [PFC](https://station.terra.money/validator/terravaloper12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69nyeu9q) validator. It will help defray the costs.

Feel free to submit patches/comments/Pull Requests.

We have also set up a [Discord](https://discord.gg/zKVWs4HhJD) channel to discuss this, and other PFC things
