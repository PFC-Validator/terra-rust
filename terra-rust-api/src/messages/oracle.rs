use crate::core_types::{Coin, Msg};

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use serde::{Deserialize, Serialize};
use std::ops::Add;

#[derive(Deserialize, Serialize, Debug)]

pub struct MsgAggregateExchangeRatePreVote2 {
    pub(crate) hash: String,
    pub(crate) feeder: String,
    pub(crate) validator: String,
}
/// used in feeder oracle
#[derive(Deserialize, Serialize, Debug)]
pub struct MsgAggregateExchangeRatePreVote {
    #[serde(rename = "type")]
    stype: String,
    value: MsgAggregateExchangeRatePreVote2,
}
impl Msg for MsgAggregateExchangeRatePreVote {}
impl MsgAggregateExchangeRatePreVote {
    /// Create a pre vote message
    pub fn create(
        hash: String,
        feeder: String,
        validator: String,
    ) -> MsgAggregateExchangeRatePreVote {
        let msg = MsgAggregateExchangeRatePreVote2 {
            hash,
            feeder,
            validator,
        };
        MsgAggregateExchangeRatePreVote {
            stype: String::from("oracle/MsgAggregateExchangeRatePrevote"),
            value: msg,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
/// the parameters to the message
pub struct MsgAggregateExchangeRateVote2 {
    /// The salt is used in the next round's 'PreVote'
    pub(crate) salt: String,
    pub(crate) exchange_rates: String,
    pub(crate) feeder: String,
    pub(crate) validator: String,
}
impl MsgAggregateExchangeRateVote2 {
    fn generate_hash(&self, previous_salt: &str) -> String {
        generate_hash(previous_salt, &self.exchange_rates, &self.validator)
    }
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
/// used in feeder oracle to submit exchange rates
#[derive(Deserialize, Serialize, Debug)]
pub struct MsgAggregateExchangeRateVote {
    #[serde(rename = "type")]
    stype: String,
    value: MsgAggregateExchangeRateVote2,
}
impl Msg for MsgAggregateExchangeRateVote {}
impl MsgAggregateExchangeRateVote {
    /// Create a vote message
    pub fn create(
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
        let msg = MsgAggregateExchangeRateVote2 {
            salt,
            exchange_rates: coins,
            feeder,
            validator,
        };
        MsgAggregateExchangeRateVote {
            stype: String::from("oracle/MsgAggregateExchangeRateVote"),
            value: msg,
        }
    }
    fn generate_hash(&self, previous_salt: &str) -> String {
        self.value.generate_hash(previous_salt)
    }
    /// Pre-Vote messages are like a 'linked list'.
    /// they use the salt of the previous 'RateVote' to hash the current prices, to ensure continuity
    pub fn gen_pre_vote(&self, previous_salt: &str) -> MsgAggregateExchangeRatePreVote {
        MsgAggregateExchangeRatePreVote::create(
            String::from(self.generate_hash(previous_salt)),
            self.value.feeder.clone(),
            self.value.validator.clone(),
        )
    }
}

#[derive(Deserialize, Serialize, Debug)]

pub struct MsgDelegateFeedConsent2 {
    pub(crate) operator: String,
    pub(crate) delegate: String,
}
/// used in feeder oracle
#[derive(Deserialize, Serialize, Debug)]
pub struct MsgDelegateFeedConsent {
    #[serde(rename = "type")]
    stype: String,
    value: MsgDelegateFeedConsent2,
}
impl Msg for MsgDelegateFeedConsent {}
impl MsgDelegateFeedConsent {
    /// Create a pre vote message
    pub fn create(operator: String, delegate: String) -> MsgDelegateFeedConsent {
        let msg = MsgDelegateFeedConsent2 { operator, delegate };
        MsgDelegateFeedConsent {
            stype: String::from("oracle/MsgDelegateFeedConsent"),
            value: msg,
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
        let vote_1 = MsgAggregateExchangeRateVote::create(salt, exchange_rates, feeder, validator);

        assert_eq!(vote_1.generate_hash(), hash);
        let pre_vote = vote_1.gen_pre_vote();
        assert_eq!(pre_vote.stype, "oracle/MsgAggregateExchangeRatePrevote");
        /*
            This example is taken from the TX log. The others are based on what the nodejs feeder generates.
               let rates_str ="22.707524482460197756uaud,21.882510617180501989ucad,16.107413560222631626uchf,114.382279464849248732ucny,14.594888140543189388ueur,12.689498975492463452ugbp,136.932658449160933002uhkd,1315.661396873891976912uinr,1917.803659404458501345ujpy,20710.846165266109229516ukrw,50292.255931832196576203umnt,12.276992042852615569usdr,23.395036036859944228usgd,0.0uthb,17.639582167170638049uusd";
               let rates: Vec<Coin> = Coin::parse_coins(rates_str)?;
               let validator2 = "terravaloper12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69nyeu9q";
               let salt2 = String::from("f560");
               let coins = rates
                   .iter()
                   .map(|f| f.to_string())
                   .collect::<Vec<String>>()
                   .join(",");
               assert_eq!(coins, rates_str);
               let hash2_eq = "cc43d5cdd5264bc214656082070a0d52dbc614e4";

               let hash2_eq_2 = generate_hash(&salt2, String::from(rates_str), validator2);
               assert_eq!(hash2_eq, hash2_eq_2);

               let vote = MsgAggregateExchangeRateVote::create(
                   salt2,
                   rates,
                   "terra1824vxwh43h9d3qczj4jvc3qphlf2evfp9w0ph9"
                       .parse()
                       .unwrap(),
                   String::from(validator2),
               );

               let hash2 = vote.generate_hash();
               assert_eq!(hash2, hash2_eq);
               let pre_vote = vote.gen_pre_vote();
               assert_eq!(pre_vote.stype, "oracle/MsgAggregateExchangeRatePrevote");

        */
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
        /*
        this was taken from a transaction dump. the others were generated directly from nodejs code

        let exchange_rates3="22.707524482460197756uaud,21.882510617180501989ucad,16.107413560222631626uchf,114.382279464849248732ucny,14.594888140543189388ueur,12.689498975492463452ugbp,136.932658449160933002uhkd,1315.661396873891976912uinr,1917.803659404458501345ujpy,20710.846165266109229516ukrw,50292.255931832196576203umnt,12.276992042852615569usdr,23.395036036859944228usgd,0.0uthb,17.639582167170638049uusd";
        let salt3 = "f560";
        let validator3 = "terravaloper12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69nyeu9q";
        let hash3 = "cc43d5cdd5264bc214656082070a0d52dbc614e4";
        assert_eq!(
            hash3,
            generate_hash(salt3, exchange_rates3.parse().unwrap(), validator3)
        );

         */
        Ok(())
    }
}
