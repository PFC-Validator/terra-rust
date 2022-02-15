use dotenv::dotenv;
use rust_decimal::prelude::Zero;
use rust_decimal::Decimal;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::ops::Div;

use terra_rust_api::staking_types::ValidatorDelegation;

use terra_rust_cli::cli_helpers;

/// VERSION number of package
pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
/// NAME of package
pub const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");

#[derive(Debug)]
pub struct Wallet {
    pub max_contrib: Decimal,
    pub min_contrib: Decimal,
    pub num_contrib: u64,
    pub sum_contrib: Decimal,
    pub validators: Vec<String>,
    pub monikers: Vec<String>,
}
#[derive(Debug)]
pub struct ValidatorDeet {
    pub moniker: String,
    pub num_delegators: usize,
    pub num_whales: usize,
    pub num_sharks: usize,
    pub total_delegations: Decimal,
    pub sharks_delegations: Decimal,
    pub whale_delegations: Decimal,
    pub whales: Vec<String>,
}
async fn run() -> anyhow::Result<()> {
    let cli = cli_helpers::gen_cli_read_only("vstat", "vstat").get_matches();
    let terra = cli_helpers::lcd_no_tx_from_args(&cli)?;
    let mut whale_set: HashSet<String> = Default::default();
    let mut shark_set: HashSet<String> = Default::default();
    let mut wallet_map: HashMap<String, Wallet> = Default::default();
    let mut validator_map: HashMap<String, ValidatorDeet> = Default::default();
    println!("chain={}", terra.chain_id);

    let validators = terra.staking().validators().await?.result;
    let prefix = format!("{}{}", std::env::temp_dir().display(), "V_");
    let wallet_prefix = format!("{}{}", std::env::temp_dir().display(), "wallet_list.csv");
    let whale_prefix = format!("{}{}", std::env::temp_dir().display(), "whale_list.csv");
    let shark_prefix = format!("{}{}", std::env::temp_dir().display(), "shark_list.csv");
    let v_prefix = format!("{}{}", std::env::temp_dir().display(), "validator_list.csv");
    println!("{}", prefix);
    let mut total_tokens: Decimal = Decimal::zero();
    for v in &validators {
        let filename = &format!("{}{}.json", prefix, v.operator_address.clone());
        let delegates: Vec<ValidatorDelegation> = if let Ok(file) = File::open(filename) {
            serde_json::from_reader(BufReader::new(file))?
        } else {
            let deets = terra
                .staking()
                .validator_delegations_limit(&v.operator_address, 16000)
                .await?;
            let file = File::create(filename)?;
            serde_json::to_writer_pretty(BufWriter::new(file), &deets.delegation_responses)?;
            deets.delegation_responses
        };

        println!(
            "{} {} {}",
            v.description.moniker,
            delegates.len(),
            Decimal::from(v.tokens).div(Decimal::from(1_000_000u64)),
        );
        total_tokens += Decimal::from(v.tokens).div(Decimal::from(1_000_000u64));
        validator_map.insert(
            v.operator_address.clone(),
            ValidatorDeet {
                moniker: v.description.moniker.clone(),
                num_delegators: delegates.len(),
                total_delegations: Decimal::from(v.tokens).div(Decimal::from(1_000_000u64)),
                sharks_delegations: Default::default(),
                whale_delegations: Default::default(),
                whales: Default::default(),
                num_whales: 0,
                num_sharks: 0,
            },
        );
        for delegate in delegates {
            let from = delegate.delegation.delegator_address;
            let amount = delegate.balance.amount.div(Decimal::from(1_000_000u64));
            wallet_map
                .entry(from)
                .and_modify(|entry| {
                    entry.num_contrib += 1;
                    entry.sum_contrib += amount;
                    if entry.max_contrib < amount {
                        entry.max_contrib = amount;
                    }
                    if entry.min_contrib > amount {
                        entry.min_contrib = amount
                    }
                    entry.validators.push(v.operator_address.clone());
                    entry.monikers.push(v.description.moniker.clone());
                })
                .or_insert(Wallet {
                    max_contrib: amount,
                    min_contrib: amount,
                    num_contrib: 1,
                    sum_contrib: amount,
                    validators: vec![v.operator_address.clone()],
                    monikers: vec![v.description.moniker.clone()],
                });
        }
    }
    let mut wallet_file = File::create(wallet_prefix)?;
    let whale_size = total_tokens.div(Decimal::from(100u64)); /* > 1% */
    let shark_size = total_tokens.div(Decimal::from(1000u64)); /* > 0.1% */
    writeln!(
        wallet_file,
        "address,max_contrib,min_contrib,num_contrib,sum_contrib, validators,monikers"
    )?;
    let mut shark_file = File::create(shark_prefix)?;
    writeln!(
        shark_file,
        "address,max_contrib,min_contrib,num_contrib,sum_contrib, validators,monikers"
    )?;
    let mut whale_file = File::create(whale_prefix)?;
    writeln!(
        whale_file,
        "address,max_contrib,min_contrib,num_contrib,sum_contrib, validators,monikers"
    )?;
    for d in wallet_map {
        let wallet = d.1;
        let address = d.0.clone();
        let validator_join = wallet.validators.join("|");
        let moniker_join = wallet.monikers.join("|");
        writeln!(
            wallet_file,
            "{},{},{},{},{},{},{}",
            address.clone(),
            wallet.max_contrib,
            wallet.min_contrib,
            wallet.num_contrib,
            wallet.sum_contrib,
            validator_join,
            moniker_join
        )?;
        if wallet.sum_contrib > whale_size {
            whale_set.insert(address.clone());
            writeln!(
                whale_file,
                "{},{},{},{},{},{},{}",
                address.clone(),
                wallet.max_contrib,
                wallet.min_contrib,
                wallet.num_contrib,
                wallet.sum_contrib,
                validator_join,
                moniker_join
            )?;
            for whale_holding in &wallet.validators {
                validator_map
                    .entry(whale_holding.into())
                    .and_modify(|entry| {
                        entry.num_whales += 1;
                        entry.whales.push(address.clone())
                    });
            }
        } else if wallet.sum_contrib > shark_size {
            shark_set.insert(address.clone());
            writeln!(
                shark_file,
                "{},{},{},{},{},{},{}",
                address.clone(),
                wallet.max_contrib,
                wallet.min_contrib,
                wallet.num_contrib,
                wallet.sum_contrib,
                validator_join,
                moniker_join
            )?;
            for shark_holding in &wallet.validators {
                validator_map
                    .entry(shark_holding.into())
                    .and_modify(|entry| {
                        entry.num_sharks += 1;
                    });
            }
        }
    }
    /* ok.. in the first pass we have calculated whales. now go through delegations and adjust whale_delegations */
    for v in validators {
        let filename = &format!("{}{}.json", prefix, v.operator_address.clone());
        let file = File::open(filename)?;
        let delegates: Vec<ValidatorDelegation> = serde_json::from_reader(BufReader::new(file))?;
        let mut whale_delegation = Decimal::zero();
        let mut shark_delegation = Decimal::zero();
        for d in delegates {
            if whale_set.contains(&d.delegation.delegator_address) {
                whale_delegation += d.balance.amount.div(Decimal::from(1_000_000u64));
            } else if shark_set.contains(&d.delegation.delegator_address) {
                shark_delegation += d.balance.amount.div(Decimal::from(1_000_000u64));
            }
        }
        validator_map.entry(v.operator_address).and_modify(|entry| {
            entry.whale_delegations = whale_delegation;
            entry.sharks_delegations = shark_delegation;
        });
    }
    let mut v_file = File::create(v_prefix)?;
    writeln!(
        v_file,
        "address,moniker,num delegators,num sharks, num whales,total delegations,shark delegations,whale delegations,whales"
    )?;
    for d in validator_map {
        let validator = d.1;
        let address = d.0.clone();
        writeln!(
            v_file,
            "{},{},{},{},{},{},{},{},{}",
            address.clone(),
            validator.moniker,
            validator.num_delegators,
            validator.num_sharks,
            validator.num_whales,
            validator.total_delegations,
            validator.sharks_delegations,
            validator.whale_delegations,
            validator.whales.join("|")
        )?;
    }
    Ok(())
}
#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    if let Err(ref err) = run().await {
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
