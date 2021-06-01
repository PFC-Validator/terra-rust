use crate::core_types::{Coin, MsgInternal};

use crate::messages::Message;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use serde::{Deserialize, Serialize};
use std::ops::Add;

#[derive(Deserialize, Serialize, Debug)]

/// used in feeder oracle
pub struct MsgAggregateExchangeRatePreVote {
    pub feeder: String,
    pub hash: String,
    pub validator: String,
}

impl MsgInternal for MsgAggregateExchangeRatePreVote {}
impl MsgAggregateExchangeRatePreVote {
    /// Create a pre vote message
    pub fn create(hash: String, feeder: String, validator: String) -> Message {
        let internal = MsgAggregateExchangeRatePreVote {
            feeder,
            hash,
            validator,
        };
        Message {
            s_type: "oracle/MsgAggregateExchangeRatePrevote".into(),
            value: Box::new(internal),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]

/// used in feeder oracle to submit exchange rates
pub struct MsgAggregateExchangeRateVote {
    pub exchange_rates: String,
    pub feeder: String,
    /// The salt is used in the next round's 'PreVote'
    pub salt: String,
    pub validator: String,
}

/// put out into a separate function to facilitate better testing
fn generate_hash<'a>(salt: &'a str, exchange_string: &'a str, validator: &'a str) -> String {
    let mut sha = Sha256::new();
    let mut to_hash: String = String::new();
    to_hash = to_hash.add(salt);
    to_hash = to_hash.add(":");
    to_hash = to_hash.add(exchange_string);
    to_hash = to_hash.add(":");
    to_hash = to_hash.add(validator);
    sha.input_str(&to_hash);
    let full_hash = sha.result_str();
    full_hash.split_at(40).0.parse().unwrap()
}
impl MsgInternal for MsgAggregateExchangeRateVote {}
impl MsgAggregateExchangeRateVote {
    fn generate_hash(&self, previous_salt: &str) -> String {
        generate_hash(previous_salt, &self.exchange_rates, &self.validator)
    }

    pub fn create_internal(
        salt: String,
        exchange_rates: Vec<Coin>,
        feeder: String,
        validator: String,
    ) -> MsgAggregateExchangeRateVote {
        let mut new_rates: Vec<Coin> = Vec::with_capacity(exchange_rates.len());
        for rate in exchange_rates {
            new_rates.push(rate);
        }
        new_rates.sort_by(|a, b| a.denom.cmp(&b.denom));
        let coins = new_rates
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join(",");
        MsgAggregateExchangeRateVote {
            salt,
            exchange_rates: coins,
            feeder,
            validator,
        }
    }
    /// Create a vote message
    pub fn create(
        salt: String,
        exchange_rates: Vec<Coin>,
        feeder: String,
        validator: String,
    ) -> Message {
        let internal =
            MsgAggregateExchangeRateVote::create_internal(salt, exchange_rates, feeder, validator);
        Message {
            s_type: "oracle/MsgAggregateExchangeRateVote".into(),
            value: Box::new(internal),
        }
    }
    /// Create a vote message from internal message
    pub fn create_from_internal(internal: MsgAggregateExchangeRateVote) -> Message {
        //  let internal =
        //      MsgAggregateExchangeRateVote::create_internal(salt, exchange_rates, feeder, validator);
        Message {
            s_type: "oracle/MsgAggregateExchangeRateVote".into(),
            value: Box::new(internal),
        }
    }

    /// Pre-Vote messages are like a 'linked list'.
    /// they use the salt of the previous 'RateVote' to hash the current prices, to ensure continuity
    pub fn gen_pre_vote(&self, previous_salt: &str) -> Message {
        MsgAggregateExchangeRatePreVote::create(
            self.generate_hash(previous_salt),
            self.feeder.clone(),
            self.validator.clone(),
        )
    }
}

#[derive(Deserialize, Serialize, Debug)]
/// used in feeder oracle

pub struct MsgDelegateFeedConsent {
    pub delegate: String,
    pub operator: String,
}
impl MsgInternal for MsgDelegateFeedConsent {}
impl MsgDelegateFeedConsent {
    /// Create a pre vote message
    pub fn create(operator: String, delegate: String) -> Message {
        let internal = MsgDelegateFeedConsent { delegate, operator };
        Message {
            s_type: "oracle/MsgDelegateFeedConsent".into(),
            value: Box::new(internal),
        }
    }
}

#[cfg(test)]
mod tst {
    use super::*;
    use crate::errors::Result;
    #[test]
    pub fn test_agg() -> Result<()> {
        let exchange_rate_str = "22.540203133218404887uaud,21.645596278923282692ucad,15.966787551658593971uchf,113.167767068332957759ucny,14.449845494375560683ueur,12.582839885411827405ugbp,135.474594500430895984uhkd,1300.213822842493250029uinr,1900.8256376511075722ujpy,20351.150811544637337767ukrw,49749.106615326838874584umnt,12.154984433357638529usdr,23.143090361112943758usgd,0.0uthb,17.444833658754882816uusd";
        let exchange_rates = Coin::parse_coins(exchange_rate_str)?;
        let salt = String::from("df59");
        let feeder = String::from("terra1824vxwh43h9d3qczj4jvc3qphlf2evfp9w0ph9");
        let validator = String::from("terravaloper1usws7c2c6cs7nuc8vma9qzaky5pkgvm2ujy8ny");
        let hash = "36681b69da96623a6ae12c2a51448b7426fdd64e";
        let coins = exchange_rates
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join(",");
        assert_eq!(coins, exchange_rate_str);

        assert_eq!(generate_hash(&salt, exchange_rate_str, &validator), hash);
        let vote_1 = MsgAggregateExchangeRateVote::create_internal(
            salt.clone(),
            exchange_rates,
            feeder,
            validator,
        );

        assert_eq!(vote_1.generate_hash(&salt), hash);
        //        let pre_vote = vote_1.gen_pre_vote(&salt);
        //        assert_eq!(pre_vote.s_type, "oracle/MsgAggregateExchangeRatePrevote");

        Ok(())
    }
    #[test]
    pub fn tst_hash() -> Result<()> {
        let exchange_rates= "22.540203133218404887uaud,21.645596278923282692ucad,15.966787551658593971uchf,113.167767068332957759ucny,14.449845494375560683ueur,12.582839885411827405ugbp,135.474594500430895984uhkd,1300.213822842493250029uinr,1900.8256376511075722ujpy,20351.150811544637337767ukrw,49749.106615326838874584umnt,12.154984433357638529usdr,23.143090361112943758usgd,0.0uthb,17.444833658754882816uusd";
        let salt = "df59";
        let validator = "terravaloper1usws7c2c6cs7nuc8vma9qzaky5pkgvm2ujy8ny";
        let hash = "36681b69da96623a6ae12c2a51448b7426fdd64e";
        assert_eq!(hash, generate_hash(salt, exchange_rates, validator));
        let exchange_rates2="22.548222362821767308uaud,21.653297230216244188ucad,15.972468127589880034uchf,113.208029274598674692ucny,14.454986380997906048ueur,12.58731653903855345ugbp,135.522792907175522923uhkd,1300.676405771192471765uinr,1901.501902950678788445ujpy,20358.286256112132944846ukrw,49766.806079087327983387umnt,12.159308866922868479usdr,23.151324082621141584usgd,0.0uthb,17.451040085807700841uusd";
        let salt2 = "6dd4";
        let validator2 = "terravaloper1usws7c2c6cs7nuc8vma9qzaky5pkgvm2ujy8ny";
        let hash2 = "54a849b1b3b510f5f0b7c5405ed2cc74cd283251";
        assert_eq!(hash2, generate_hash(salt2, exchange_rates2, validator2));

        Ok(())
    }
}
