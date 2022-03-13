use crate::errors::TerraRustCLIError;
use crate::errors::TerraRustCLIError::MissingEnv;
use clap::{Arg, ArgMatches, Parser};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use secp256k1::{Context, Secp256k1, Signing};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use terra_rust_api::core_types::Coin;
use terra_rust_api::{GasOptions, PrivateKey, Terra};
use terra_rust_wallet::Wallet;

/// your terra swiss army knife
#[derive(Parser)]

pub struct Cli<T: clap::FromArgMatches + clap::Subcommand> {
    #[clap(
        name = "lcd",
        env = "TERRARUST_LCD",
        default_value = "https://lcd.terra.dev",
        short,
        long = "lcd-client-url",
        help = "https://lcd.terra.dev is main-net, https://bombay-lcd.terra.dev"
    )]
    // Terra cli Client daemon
    pub lcd: String,
    #[clap(
        name = "fcd",
        env = "TERRARUST_FCD",
        default_value = "https://fcd.terra.dev",
        long = "fcd-client-url",
        help = "https://fcd.terra.dev is main-net. currently only used to fetch gas prices"
    )]
    // Terra cli Client daemon
    pub fcd: String,
    #[clap(
        name = "chain",
        env = "TERRARUST_CHAIN",
        default_value = "columbus-5",
        short,
        long = "chain",
        help = "bombay-12 is testnet, columbus-5 is main-net"
    )]
    pub chain_id: String,
    // Wallet name
    #[clap(
        name = "wallet",
        env = "TERRARUST_WALLET",
        default_value = "default",
        short,
        long = "wallet",
        help = "the default wallet to look for keys in"
    )]
    pub wallet: String,
    #[clap(
        name = "seed",
        env = "TERRARUST_SEED_PHRASE",
        default_value = "",
        short,
        long = "seed",
        help = "the seed phrase to use with this private key"
    )]
    pub seed: String,
    #[clap(
        name = "fees",
        default_value = "",
        short,
        long = "fees",
        help = "the fees to use. This will override gas parameters if specified."
    )]
    pub fees: String,
    #[clap(
        name = "gas",
        default_value = "auto",
        long = "gas",
        help = "the gas amount to use 'auto' to estimate"
    )]
    pub gas: String,
    #[clap(
        name = "gas-prices",
        env = "TERRARUST_GAS_PRICES",
        default_value = "auto",
        long = "gas-prices",
        help = "the gas price to use to calculate fee. Format is NNNtoken eg. 1000uluna. note we only support a single price for now. if auto. it will use FCD"
    )]
    pub gas_price: String,
    #[clap(
        name = "gas-denom",
        env = "TERRARUST_GAS_DENOM",
        default_value = "ukrw",
        long = "gas-denom",
        help = "the denomination/currency to use to pay fee. Format is uXXXX."
    )]
    pub gas_price_denom: String,
    #[clap(
        name = "gas-adjustment",
        default_value = "1.4",
        env = "TERRARUST_GAS_ADJUSTMENT",
        long = "gas-adjustment",
        help = "the adjustment to multiply the estimate to calculate the fee"
    )]
    pub gas_adjustment: f64,
    #[clap(short, long, parse(from_flag))]
    pub debug: std::sync::atomic::AtomicBool,
    #[clap(subcommand)]
    pub cmd: T,
}
impl<T: clap::FromArgMatches + clap::Subcommand> Cli<T> {
    pub async fn gas_opts(&self) -> Result<GasOptions, TerraRustCLIError> {
        if self.gas_price == "auto" {
            let client = reqwest::Client::new();
            let gas_opts = GasOptions::create_with_fcd(
                &client,
                &self.fcd,
                &self.gas_price_denom,
                self.gas_adjustment,
            )
            .await?;
            if let Some(gas_price) = &gas_opts.gas_price {
                log::info!("Using Gas price of {}", gas_price);
            }

            Ok(gas_opts)
        } else {
            let fees = Coin::parse(&self.fees)?;
            let gas_str = &self.gas;
            let (estimate_gas, gas) = if gas_str == "auto" {
                (true, None)
            } else {
                let g = &self.gas.parse::<u64>()?;
                (false, Some(*g))
            };

            let gas_price = Coin::parse(&self.gas_price)?;
            let gas_adjustment = Some(self.gas_adjustment);

            Ok(GasOptions {
                fees,
                estimate_gas,
                gas,
                gas_price,
                gas_adjustment,
            })
        }
    }
}
#[allow(dead_code)]
pub fn gen_cli_read_only<'a>(app_name: &'a str, bin_name: &'a str) -> clap::Command<'a> {
    clap::Command::new(app_name)
        .bin_name(bin_name)
        .arg(
            Arg::new("lcd")
                .long("lcd")
                .value_name("lcd")
                .takes_value(true)
                .env("TERRARUST_LCD")
                .default_value("https://lcd.terra.dev")
                .help("https://lcd.terra.dev is main-net, https://bombay-lcd.terra.dev"),
        )
        .arg(
            Arg::new("fcd")
                .long("fcd")
                .value_name("fcd")
                .takes_value(true)
                .env("TERRARUST_FCD")
                .default_value("https://fcd.terra.dev")
                .help("https://fcd.terra.dev is main-net, https://bombay-fcd.terra.dev"),
        )
        .arg(
            Arg::new("chain")
                .long("chain")
                .takes_value(true)
                .value_name("chain")
                .env("TERRARUST_CHAIN")
                .default_value("columbus-5")
                .help("bombay-12 is testnet, columbus-5 is main-net"),
        )
}
#[allow(dead_code)]
pub fn gen_cli<'a>(app_name: &'a str, bin_name: &'a str) -> clap::Command<'a> {
    gen_cli_read_only(app_name,bin_name).args(&[
        Arg::new("wallet").long("wallet").takes_value(true).value_name("wallet").env("TERRARUST_WALLET").default_value("default").help( "the default wallet to look for keys in"),
        Arg::new("seed").long("seed").takes_value(true).value_name("seed").env("TERRARUST_SEED_PHRASE").default_value("").help(  "the seed phrase to use with this private key"),
        Arg::new("fees").long("fees").takes_value(true).value_name("fees").default_value("").help(   "the fees to use. This will override gas parameters if specified."),
        Arg::new("gas").long("gas").takes_value(true).value_name("gas").default_value("auto").help(   "the gas amount to use 'auto' to estimate"),
        Arg::new("gas-prices").long("gas-prices").takes_value(true).value_name("gas-prices").default_value("auto").help(    "the gas price to use to calculate fee. Format is NNNtoken eg. 1000uluna. note we only support a single price for now. if auto. it will use FCD"),
        Arg::new("gas-denom").long("gas-denom").takes_value(true).value_name("gas-denom").env("TERRARUST_GAS_DENOM").default_value("ukrw").help(    "the denomination/currency to use to pay fee. Format is uXXXX."),
        Arg::new("gas-adjustment").long("gas-adjustment").takes_value(true).value_name("gas-adjustment").default_value("1.4").help(    "the adjustment to multiply the estimate to calculate the fee"),
        Arg::new("sender").long("sender").takes_value(true).value_name("sender").help( "wallet that is sending the command")
        .env("TERRARUST_SENDER"),
        Arg::new("phrase")
            .long("phrase")
            .takes_value(true)
            .value_name("phrase")
            .required(false)
            .help("the phrase words for the key"),
    ])
}
#[allow(dead_code)]
pub async fn gas_opts(arg_matches: &ArgMatches) -> Result<GasOptions, TerraRustCLIError> {
    let gas_price = arg_matches
        .value_of("gas-prices")
        .expect("gas-prices should be in the CLI");
    let gas_adjustment = arg_matches
        .value_of("gas-adjustment")
        .unwrap()
        .parse::<f64>()?;
    if gas_price == "auto" {
        let fcd = arg_matches.value_of("fcd").unwrap();
        let gas_price_denom = arg_matches.value_of("gas-denom").unwrap();

        let client = reqwest::Client::new();
        let gas_opts =
            GasOptions::create_with_fcd(&client, fcd, gas_price_denom, gas_adjustment).await?;
        if let Some(gas_price) = &gas_opts.gas_price {
            log::info!("Using Gas price of {}", gas_price);
        }

        Ok(gas_opts)
    } else {
        let gas_str = arg_matches.value_of("gas").unwrap();
        let fees = Coin::parse(arg_matches.value_of("fees").unwrap())?;

        let (estimate_gas, gas) = if gas_str == "auto" {
            (true, None)
        } else {
            let g = &gas_str.parse::<u64>()?;
            (false, Some(*g))
        };

        let gas_price = Coin::parse(gas_price)?;
        let gas_adjustment = Some(gas_adjustment);

        Ok(GasOptions {
            fees,
            estimate_gas,
            gas,
            gas_price,
            gas_adjustment,
        })
    }
}
#[allow(dead_code)]
pub fn wallet_from_args(cli: &ArgMatches) -> Result<Wallet, TerraRustCLIError> {
    let wallet = get_arg_value(cli, "wallet")?;
    Ok(Wallet::create(wallet))
}

#[allow(dead_code)]
pub async fn lcd_from_args(cli: &ArgMatches) -> Result<Terra, TerraRustCLIError> {
    let gas_opts = gas_opts(cli).await?;
    let lcd = get_arg_value(cli, "lcd")?;
    let chain_id = get_arg_value(cli, "chain")?;

    Ok(Terra::lcd_client(lcd, chain_id, &gas_opts, None))
}
#[allow(dead_code)]
pub fn lcd_no_tx_from_args(cli: &ArgMatches) -> Result<Terra, TerraRustCLIError> {
    let lcd = get_arg_value(cli, "lcd")?;
    let chain_id = get_arg_value(cli, "chain")?;

    Ok(Terra::lcd_client_no_tx(lcd, chain_id))
}

pub fn get_private_key<C: Context + Signing>(
    secp: &Secp256k1<C>,
    matches: &ArgMatches,
) -> Result<PrivateKey, TerraRustCLIError> {
    if let Some(phrase) = matches.value_of("phrase") {
        if let Some(seed) = matches.value_of("seed") {
            Ok(PrivateKey::from_words_seed(secp, phrase, seed)?)
        } else {
            Ok(PrivateKey::from_words(secp, phrase, 0, 0)?)
        }
    } else {
        let wallet = wallet_from_args(matches)?;
        let sender = get_arg_value(matches, "sender")?;
        Ok(wallet.get_private_key(secp, sender, matches.value_of("seed"))?)
    }
}

pub fn get_arg_value<'a>(cli: &'a ArgMatches, id: &str) -> Result<&'a str, TerraRustCLIError> {
    if let Some(val) = cli.value_of(id) {
        Ok(val)
    } else {
        Err(TerraRustCLIError::MissingArgument(id.to_string()))
    }
}
/// expand json with environmental values
pub fn expand_block(
    in_str: &str,
    sender_account: Option<String>,
) -> Result<String, TerraRustCLIError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"###(.*?)###").expect("unable to compile regex");
    }
    let mut missing_env: Option<String> = None;
    let caps = RE.replace_all(&in_str, |captures: &Captures| match &captures[1] {
        "" => String::from("%"),
        "SENDER" => {
            if let Some(sender) = sender_account.clone() {
                sender.clone()
            } else {
                missing_env = Some("SENDER".to_string());
                "-".to_string()
            }
        }
        varname => {
            if varname.starts_with("E:") {
                let env_var = varname.split_at(2).1;
                if let Ok(value) = std::env::var(env_var) {
                    value
                } else {
                    missing_env = Some(env_var.to_string());
                    format!("###_err_{}###", env_var)
                }
            } else {
                format!("###{}###", varname)
            }
        }
    });
    if let Some(env) = missing_env {
        Err(MissingEnv(env))
    } else {
        Ok(caps.to_string())
    }
}
/// convert a input parameter into json.
/// input can either be a json string, a file, or '-' to read stdin.
///
pub fn get_json_block(in_str: &str) -> Result<serde_json::Value, TerraRustCLIError> {
    if in_str.starts_with('{') {
        Ok(serde_json::from_str::<serde_json::Value>(in_str)?)
    } else if in_str == "-" {
        let input = std::io::stdin();
        let mut input = input.lock();
        let mut str_buf = String::new();
        input.read_to_string(&mut str_buf)?;

        Ok(serde_json::from_str(&str_buf)?)
    } else {
        let p = Path::new(in_str);
        let file = File::open(p)?;
        let rdr = BufReader::new(file);
        Ok(serde_json::from_reader(rdr)?)
    }
}
/// convert a input parameter into json, expanding the JSON returned with environment variables
/// input can either be a json string, a file, or '-' to read stdin.
///
pub fn get_json_block_expanded(
    in_str: &str,
    sender: Option<String>,
) -> Result<serde_json::Value, TerraRustCLIError> {
    let json = get_json_block(in_str)?;
    let in_str = serde_json::to_string(&json)?;
    let out_str = expand_block(&in_str, sender)?;
    Ok(serde_json::from_str(&out_str)?)
}

#[cfg(test)]
mod tst {
    use super::*;
    use std::env;
    #[test]
    pub fn test() -> anyhow::Result<()> {
        assert_eq!(
            "mary had a little lamb",
            expand_block("mary had a little lamb", Some("sender".into()))?
        );
        assert_eq!(
            "mary had a ###little lamb",
            expand_block("mary had a ###little lamb", Some("sender".into()))?
        );
        assert_eq!(
            "mary had a sender lamb",
            expand_block("mary had a ###SENDER### lamb", Some("sender".into()))?
        );
        env::set_var("FOO", "BAR");
        assert_eq!(
            "mary had a BAR lamb",
            expand_block("mary had a ###E:FOO### lamb", Some("sender".into()))?
        );
        assert_eq!(
            "mary had a BAR ###lamb",
            expand_block("mary had a ###E:FOO### ###lamb", Some("sender".into()))?
        );
        assert_eq!(
            "mary BAR a ###FOO### ###lamb",
            expand_block(
                "mary ###E:FOO### a ###FOO### ###lamb",
                Some("sender".into())
            )?
        );
        assert_eq!(
            "mary BAR a BAR ###lamb",
            expand_block(
                "mary ###E:FOO### a ###E:FOO### ###lamb",
                Some("sender".into())
            )?
        );
        assert_eq!(
            "mary BAR sender BAR ###lamb",
            expand_block(
                "mary ###E:FOO### ###SENDER### ###E:FOO### ###lamb",
                Some("sender".into())
            )?
        );
        env::set_var("XYZ", "aXYZc");
        assert_eq!(
            "mary BAR sender aXYZc ###lamb",
            expand_block(
                "mary ###E:FOO### ###SENDER### ###E:XYZ### ###lamb",
                Some("sender".into())
            )?
        );
        assert_eq!(
            "mary BAR xxx aXYZc ###lamb",
            expand_block("mary ###E:FOO### xxx ###E:XYZ### ###lamb", None)?
        );
        assert!(expand_block(
            "mary ###E:FOO### ###SENDER### ###E:AAA### ###lamb",
            Some("sender".into())
        )
        .is_err());
        assert!(expand_block("mary ###E:FOO### ###SENDER### ###lamb", None).is_err());
        Ok(())
    }
}
