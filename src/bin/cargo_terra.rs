use anyhow::Result;
use clap::{App, Arg, ArgMatches, Subcommand};
use dotenv::dotenv;
use secp256k1::{Context, Secp256k1, Signing};
use terra_rust_api::core_types::Coin;
use terra_rust_api::messages::wasm::{MsgInstantiateContract, MsgMigrateContract};
use terra_rust_api::{Message, MsgExecuteContract, PrivateKey, Terra};
use terra_rust_cli::cli_helpers;
/// VERSION number of package
pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
/// NAME of package
pub const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");

#[derive(Subcommand)]
#[allow(clippy::upper_case_acronyms)]
enum Command {
    Migrate {
        contract: String,
        wasm: String,
        migrate: Option<String>,
    },
    Exec {
        contract: String,
        exec: String,
        coins: Option<String>,
    },
    Instantiate {
        wasm: String,
        json: String,
        admin: Option<String>,
        coins: Option<String>,
    },
    Query {
        contract: String,
        query: String,
    },
}
async fn run(args: Vec<String>) -> Result<()> {
    let memo = Some(format!(
        "PFC-{}/{}",
        NAME.unwrap_or("TERRARUST"),
        VERSION.unwrap_or("DEV")
    ));
    let cli: App = cli_helpers::gen_cli("terra", "cargo-terra").args(&[
        Arg::new("phrase")
            .long("phrase")
            .takes_value(true)
            .value_name("phrase")
            .required(false)
            .help("the phrase words for the key"),
        Arg::new("retries")
            .long("retries")
            .takes_value(true)
            .value_name("retries")
            .required(false)
            .default_value("5")
            .help("amount of times to retry fetching hash"),
        Arg::new("sleep")
            .long("sleep")
            .takes_value(true)
            .value_name("sleep")
            .required(false)
            .default_value("3")
            .help("amount of seconds before retying to fetch hash"),
    ]);
    let matches: ArgMatches = Command::augment_subcommands(cli).get_matches_from(args);

    let sleep = cli_helpers::get_arg_value(&matches, "sleep")?.parse::<u64>()?;
    let retries = cli_helpers::get_arg_value(&matches, "retries")?.parse::<usize>()?;

    match matches.subcommand() {
        Some(("migrate", migrate)) => {
            let contract = cli_helpers::get_arg_value(&migrate, "contract")?;

            if !contract.starts_with("terra1") {
                anyhow::bail!("invalid contract address");
            }
            let terra = cli_helpers::lcd_from_args(&matches).await?;
            let secp = Secp256k1::new();
            let private = get_private_key(&secp, &matches)?;
            let wasm = cli_helpers::get_arg_value(&migrate, "wasm")?;
            let code_id = if let Ok(code_id) = wasm.parse::<u64>() {
                code_id
            } else {
                let hash = terra
                    .wasm()
                    .store(&secp, &private, wasm, memo.clone())
                    .await?
                    .txhash;
                let code_id = get_attribute_tx(
                    &terra,
                    &hash,
                    retries,
                    tokio::time::Duration::from_secs(sleep),
                    "store_code",
                    "code_id",
                )
                .await?;
                code_id.parse::<u64>()?
            };

            let json = if let Some(migrate_json) = migrate.value_of("migrate") {
                let json_block = cli_helpers::get_json_block(migrate_json)?.to_string();
                Some(MsgMigrateContract::replace_parameters(
                    &private.public_key(&secp).account()?,
                    contract,
                    code_id,
                    &json_block,
                ))
            } else {
                None
            };
            let hash = terra
                .wasm()
                .migrate(&secp, &private, contract, code_id, json, memo)
                .await?
                .txhash;
            let tx = terra
                .tx()
                .get_and_wait(&hash, retries, tokio::time::Duration::from_secs(sleep))
                .await?;
            let codes = tx.get_attribute_from_result_logs("migrate_contract", "contract_address");
            let contract = if let Some(code) = codes.first() {
                code.1.clone()
            } else {
                panic!(
                    "{}/{} not present in TX log",
                    "migrate_contract", "contract_address"
                );
            };

            let codes = tx.get_attribute_from_result_logs("migrate_contract", "code_id");
            let code_id = if let Some(code) = codes.first() {
                code.1.clone()
            } else {
                panic!("{}/{} not present in TX log", "migrate_contract", "code_id");
            };

            println!("Contract: {} Migrated to {}", contract, code_id);
        }
        Some(("instantiate", instantiate)) => {
            let terra = cli_helpers::lcd_from_args(&matches).await?;
            let secp = Secp256k1::new();
            let private = get_private_key(&secp, &matches)?;
            let wasm = cli_helpers::get_arg_value(&instantiate, "wasm")?;
            let coins = if let Some(coin_str) = instantiate.value_of("coins") {
                Coin::parse_coins(coin_str)?
            } else {
                vec![]
            };
            let code_id = if let Ok(code_id) = wasm.parse::<u64>() {
                code_id
            } else {
                let hash = terra
                    .wasm()
                    .store(&secp, &private, wasm, memo.clone())
                    .await?
                    .txhash;
                let code_id = get_attribute_tx(
                    &terra,
                    &hash,
                    retries,
                    tokio::time::Duration::from_secs(sleep),
                    "store_code",
                    "code_id",
                )
                .await?;
                code_id.parse::<u64>()?
            };
            let admin: Option<String> = if let Some(admin) = instantiate.value_of("admin") {
                if admin.starts_with("terra1") {
                    Some(admin.to_string())
                } else if admin == "same" {
                    Some(private.public_key(&secp).account()?)
                } else if admin == "none" {
                    None
                } else {
                    let wallet = cli_helpers::wallet_from_args(&matches)?;
                    let seed = matches.value_of("seed");
                    let admin_key = wallet.get_public_key(&secp, &admin, seed)?;
                    let admin_account = admin_key.account()?;
                    Some(admin_account)
                }
            } else {
                None
            };

            let init_json = cli_helpers::get_arg_value(instantiate, "json")?;
            let json = cli_helpers::get_json_block(init_json)?.to_string();
            let init_json_parsed = MsgInstantiateContract::replace_parameters(
                &private.public_key(&secp).account()?,
                admin.clone(),
                code_id,
                &json,
            );

            let hash = terra
                .wasm()
                .instantiate(
                    &secp,
                    &private,
                    code_id,
                    init_json_parsed,
                    coins,
                    admin,
                    memo,
                )
                .await?
                .txhash;
            let tx = terra
                .tx()
                .get_and_wait(&hash, retries, tokio::time::Duration::from_secs(sleep))
                .await?;
            let codes =
                tx.get_attribute_from_result_logs("instantiate_contract", "contract_address");
            let contract = if let Some(code) = codes.first() {
                code.1.clone()
            } else {
                panic!(
                    "{}/{} not present in TX log",
                    "migrate_contract", "contract_address"
                );
            };

            let codes = tx.get_attribute_from_result_logs("instantiate_contract", "code_id");
            let code_id = if let Some(code) = codes.first() {
                code.1.clone()
            } else {
                panic!("{}/{} not present in TX log", "migrate_contract", "code_id");
            };

            println!("Contract: {} running  code {}", contract, code_id);
        }
        Some(("exec", exec)) => {
            let contract = cli_helpers::get_arg_value(&exec, "contract")?;

            if !contract.starts_with("terra1") {
                anyhow::bail!("invalid contract address");
            }
            let terra = cli_helpers::lcd_from_args(&matches).await?;
            let secp = Secp256k1::new();
            let private = get_private_key(&secp, &matches)?;
            let coins = if let Some(coin_str) = exec.value_of("coins") {
                Coin::parse_coins(coin_str)?
            } else {
                vec![]
            };
            let exec_str = cli_helpers::get_arg_value(exec, "exec")?;
            let json = cli_helpers::get_json_block(exec_str)?;
            let exec_message = MsgExecuteContract::create_from_value(
                &private.public_key(&secp).account()?,
                contract,
                &json,
                &coins,
            )?;
            let messages: Vec<Message> = vec![exec_message];

            let resp = terra
                .submit_transaction_sync(&secp, &private, messages, memo)
                .await?
                .txhash;
            println!("{}", resp);
        }
        Some(("query", query)) => {
            let contract = cli_helpers::get_arg_value(&query, "contract")?;

            if !contract.starts_with("terra1") {
                anyhow::bail!("invalid contract address");
            }
            let terra = cli_helpers::lcd_no_tx_from_args(&matches)?;
            let query_str = cli_helpers::get_arg_value(query, "query")?;
            let query_json = cli_helpers::get_json_block(query_str)?.to_string();
            let result = terra
                .wasm()
                .query::<serde_json::Value>(contract, &query_json)
                .await?;

            println!("{}", serde_json::to_string_pretty(&result)?)
        }
        _ => {
            eprintln!("got {:?}", matches.subcommand());
            panic!("no command found. This is a coding bug")
        }
    }
    Ok(())
}

fn get_private_key<C: Context + Signing>(
    secp: &Secp256k1<C>,
    matches: &ArgMatches,
) -> Result<PrivateKey> {
    if let Some(phrase) = matches.value_of("phrase") {
        if let Some(seed) = matches.value_of("seed") {
            Ok(PrivateKey::from_words_seed(secp, phrase, seed)?)
        } else {
            Ok(PrivateKey::from_words(secp, phrase, 0, 0)?)
        }
    } else {
        let wallet = cli_helpers::wallet_from_args(matches)?;
        let sender = cli_helpers::get_arg_value(matches, "sender")?;
        Ok(wallet.get_private_key(secp, sender, matches.value_of("seed"))?)
    }
}

async fn get_attribute_tx(
    terra: &Terra,
    hash: &str,
    retries: usize,
    sleep: tokio::time::Duration,
    event_type: &str,
    attribute_key: &str,
) -> Result<String> {
    let tx = terra.tx().get_and_wait(hash, retries, sleep).await?;
    let codes = tx.get_attribute_from_result_logs(event_type, attribute_key);
    if let Some(code) = codes.first() {
        Ok(code.1.clone())
    } else {
        panic!("{}/{} not present in TX log", event_type, attribute_key)
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    // in case we are invoked by cargo-terra
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "terra" {
        args.remove(1);
    }
    println!("args[1]={}", args[1]);
    if let Err(ref err) = run(args).await {
        log::error!("{}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| log::error!("because: {}", cause));

        // The backtrace is not always generated. Try to run this example
        // with `$env:RUST_BACKTRACE=1`.
        //    if let Some(backtrace) = e.backtrace() {
        //        log::debug!("backtrace: {:?}", backtrace);
        //    }

        ::std::process::exit(1);
    }
}
