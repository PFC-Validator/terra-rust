use crate::errors::{ErrorKind, Result};

use hkd32;
use hkd32::mnemonic::{Phrase, Seed};

use rand_core::OsRng;

pub static LUNA_COIN_TYPE: u64 = 330;

pub struct MnemonicKey {
    account: u64,
    pub index: u64,
    pub coin_type: u64,
    mnemonic: Phrase,
}

impl MnemonicKey {
    pub fn from_mnemonic(words: &str) -> Result<MnemonicKey> {
        // let m = Mnemonic::parse(words)?;
        let m = hkd32::mnemonic::Phrase::new(words, hkd32::mnemonic::Language::English);
        match m {
            Ok(phrase) => Ok(MnemonicKey {
                account: 0,
                index: 0,
                coin_type: LUNA_COIN_TYPE,
                mnemonic: phrase,
            }),
            Err(_) => Err(ErrorKind::Phrasing.into()),
        }
    }
    pub fn from_mnemonic_ex(
        account: u64,
        index: u64,
        coin_type: u64,
        words: &str,
    ) -> Result<MnemonicKey> {
        let m = hkd32::mnemonic::Phrase::new(words, hkd32::mnemonic::Language::English);
        match m {
            Ok(phrase) =>
            // let m = Mnemonic::parse(words)?;
            {
                Ok(MnemonicKey {
                    account,
                    index,
                    coin_type,
                    mnemonic: phrase,
                })
            }
            Err(_) => Err(ErrorKind::Phrasing.into()),
        }
    }
    fn seed(&self) -> Seed {
        self.mnemonic.to_seed("")
        //  self.mnemonic.to_seed_normalized("")
    }
    pub fn new() -> Result<MnemonicKey> {
        //   let mut rng = rand_hc::Hc128Rng::from_entropy();
        // let mut rng = rand::thread_rng();

        let m = hkd32::mnemonic::Phrase::random(&mut OsRng, hkd32::mnemonic::Language::English);
        /*
                let m = Mnemonic::generate_in_with(&mut rng, Language::English, 24)
                    .unwrap()
                    .to_owned();
        */
        Ok(MnemonicKey {
            account: 0,
            index: 0,
            coin_type: LUNA_COIN_TYPE,
            mnemonic: m,
        })
    }
    pub fn new_ex(language: hkd32::mnemonic::Language) -> Result<MnemonicKey> {
        let m = hkd32::mnemonic::Phrase::random(&mut OsRng, language);

        Ok(MnemonicKey {
            account: 0,
            index: 0,
            coin_type: LUNA_COIN_TYPE,
            mnemonic: m,
        })
    }
    pub fn to_string(&self) -> String {
        self.mnemonic.phrase().to_string()
    }
}

#[cfg(test)]
mod tst {
    use super::*;

    use crate::keys::{encode_hex, PublicKey};
    use bitcoin::util::bip32::{ExtendedPrivKey, IntoDerivationPath};
    use bitcoin::Network;

    use crate::keys::address::AccAddress;
    use bitcoin::secp256k1::Secp256k1;

    #[test]
    pub fn tst_gen_mnemonic() -> Result<()> {
        // this test just makes sure the default will call it.
        let _y = MnemonicKey::new()?;
        Ok(())
    }
    #[test]
    pub fn tst_from_words() -> Result<()> {
        let y = MnemonicKey::from_mnemonic("a b c");
        assert!(!y.is_ok());
        let str_1 = "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius";
        let seed_1 = "a2ae8846397b55d266af35acdbb18ba1d005f7ddbdd4ca7a804df83352eaf373f274ba0dc8ac1b2b25f19dfcb7fa8b30a240d2c6039d88963defc2f626003b2f";
        let x = MnemonicKey::from_mnemonic(str_1);

        assert!(x.is_ok());
        let x1mk = x?;
        assert_eq!(str_1, x1mk.to_string());
        assert_eq!(encode_hex(&x1mk.seed().as_bytes()).unwrap(), seed_1);

        // let str_2 = "regret february current outer produce scare furnace december dinner pottery wave elegant resemble domain brand drastic park aunt fatigue recipe spike wink sport bleak";
        let str_2 = "wonder caution square unveil april art add hover spend smile proud admit modify old copper throw crew happy nature luggage reopen exhibit ordinary napkin";
        let x2 = MnemonicKey::from_mnemonic(str_2);

        assert!(x2.is_ok());
        let x2mk = x2?;
        assert_eq!(str_2, x2mk.to_string());
        let seed = x2mk.seed();
        let seed_2="7af4a96f0a674c61f4b8de3506fca4cab0ff7ab66b92ef9a18276f8a715805939b19f94744cd562808eda935172ad5730fc420ecb44ae5c5fc0a1b8c83a41bed";
        //     let seed_2="02a612a749a1627b0bdcb8064323dfd6635ded6be59bfcc40f6a9a8b7431f674623d35fee973ea9d78313925b8d89f0cc2d6015f92966520efd7003dabd3d4b4";
        assert_eq!(encode_hex(&seed.as_bytes()).unwrap(), seed_2);
        let secp = Secp256k1::new();
        let root = ExtendedPrivKey::new_master(Network::Bitcoin, &seed.as_bytes()).unwrap();

        let root_key_2 = "xprv9s21ZrQH143K2ep3BpYRRMjSqjLHZAPAzxfVVS3NBuGKBVtCrK3C8mE8TcmTjYnLm7SJxdLigDFWGAMnctKxc3p5QKNWXdprcFSQzGzQqTW";
        assert_eq!(root.to_string(), root_key_2);
        let path = format!(
            "m/44'/{}'/{}'/0/{}",
            LUNA_COIN_TYPE, 0, /*account*/ 0 /*index*/
        );
        let derivation_path = path.into_derivation_path()?;

        let mk = root.derive_priv(&secp, &derivation_path)?;

        assert_eq!(
            "4804e2bdce36d413206ccf47cc4c64db2eff924e7cc9e90339fa7579d2bd9d5b",
            mk.private_key.key.to_string()
        );
        let pubK = mk.private_key.public_key(&secp);
        assert_eq!(
            "02cf7ed0b5832538cd89b55084ce93399b186e381684b31388763801439cbdd20a",
            pubK.key.to_string()
        );
        //        eprintln!("Pub:{}", pubK.key);
        let raw_pub_key = PublicKey::pubkey_from_public_key(&pubK.key.serialize());
        assert_eq!(
            "eb5ae9872102cf7ed0b5832538cd89b55084ce93399b186e381684b31388763801439cbdd20a",
            encode_hex(&raw_pub_key)?
        );
        let raw_addr = PublicKey::address_from_public_key(&pubK.key.serialize());
        assert_eq!(
            "94c4c52a9777e3c3628e5cfe819f6e26a7f5bd82",
            encode_hex(&raw_addr)?
        );
        let acc_address: AccAddress = AccAddress::from_hex(&raw_addr).unwrap();
        assert_eq!(
            "terra1jnzv225hwl3uxc5wtnlgr8mwy6nlt0vztv3qqm",
            acc_address.to_string()
        );

        Ok(())
    }
}
