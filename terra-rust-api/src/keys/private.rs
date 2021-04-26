use crate::core_types::StdSignature;
use crate::errors::{ErrorKind, Result};
use crate::keys::PublicKey;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::secp256k1::{All, Message};
use bitcoin::util::bip32::{ExtendedPrivKey, IntoDerivationPath};
use bitcoin::Network;
use crypto::sha2::Sha256;

use crypto::digest::Digest;
use hkd32::mnemonic::{Phrase, Seed};

use rand_core::OsRng;
/// This is the coin type used in most derivations
pub static LUNA_COIN_TYPE: u32 = 330;

/// The Private key structure that is used to generate signatures and public keys
/// WARNING: No Security Audit has been performed
pub struct PrivateKey {
    #[allow(missing_docs)]
    pub account: u32,
    #[allow(missing_docs)]
    pub index: u32,
    #[allow(missing_docs)]
    pub coin_type: u32,
    /// The 24 words used to generate this private key
    mnemonic: Option<Phrase>,
    #[allow(dead_code)]
    /// This is used for testing
    root_private_key: ExtendedPrivKey,
    /// The private key
    private_key: ExtendedPrivKey,
}
impl PrivateKey {
    /// Generate a new private key
    pub fn new(secp: &Secp256k1<All>) -> Result<PrivateKey> {
        let phrase =
            hkd32::mnemonic::Phrase::random(&mut OsRng, hkd32::mnemonic::Language::English);

        PrivateKey::gen_private_key_phrase(secp, phrase, 0, 0, LUNA_COIN_TYPE, "")
    }
    /// generate a new private key with a seed phrase
    pub fn new_seed(secp: &Secp256k1<All>, seed_phrase: &str) -> Result<PrivateKey> {
        let phrase =
            hkd32::mnemonic::Phrase::random(&mut OsRng, hkd32::mnemonic::Language::English);

        PrivateKey::gen_private_key_phrase(secp, phrase, 0, 0, LUNA_COIN_TYPE, seed_phrase)
    }
    /// for private key recovery. This is also used by wallet routines to re-hydrate the structure
    pub fn from_words(secp: &Secp256k1<All>, words: &str) -> Result<PrivateKey> {
        match hkd32::mnemonic::Phrase::new(words, hkd32::mnemonic::Language::English) {
            Ok(phrase) => {
                PrivateKey::gen_private_key_phrase(secp, phrase, 0, 0, LUNA_COIN_TYPE, "")
            }
            Err(_) => Err(ErrorKind::Phrasing.into()),
        }
    }
    /// for private key recovery with seed phrase
    pub fn from_words_seed(
        secp: &Secp256k1<All>,
        words: &str,
        seed_pass: &str,
    ) -> Result<PrivateKey> {
        match hkd32::mnemonic::Phrase::new(words, hkd32::mnemonic::Language::English) {
            Ok(phrase) => {
                PrivateKey::gen_private_key_phrase(secp, phrase, 0, 0, LUNA_COIN_TYPE, seed_pass)
            }
            Err(_) => Err(ErrorKind::Phrasing.into()),
        }
    }
    /// generate the public key for this private key
    pub fn public_key(&self, secp: &Secp256k1<All>) -> PublicKey {
        let x = &self.private_key.private_key.public_key(secp);
        PublicKey::from_bitcoin_public_key(x)
    }

    fn gen_private_key_phrase(
        secp: &Secp256k1<All>,
        phrase: Phrase,
        account: u32,
        index: u32,
        coin_type: u32,
        seed_phrase: &str,
    ) -> Result<PrivateKey> {
        let seed = phrase.to_seed(seed_phrase);
        let root_private_key =
            ExtendedPrivKey::new_master(Network::Bitcoin, &seed.as_bytes()).unwrap();
        let path = format!("m/44'/{}'/{}'/0/{}", coin_type, account, index);
        let derivation_path = path.into_derivation_path()?;

        let private_key = root_private_key.derive_priv(secp, &derivation_path)?;
        Ok(PrivateKey {
            account,
            index,
            coin_type,
            mnemonic: Some(phrase),
            root_private_key,
            private_key,
        })
    }

    /// the words used to generate this private key
    pub fn words(&self) -> Option<&str> {
        match &self.mnemonic {
            Some(phrase) => Some(phrase.phrase()),
            None => None,
        }
    }
    /// signs a blob of data and returns a [StdSignature]
    pub fn sign(&self, secp: &Secp256k1<All>, blob: &str) -> Result<StdSignature> {
        let pub_k = &self.private_key.private_key.public_key(secp);
        let priv_k = self.private_key.private_key.key;
        let mut sha = Sha256::new();
        let mut sha_result: [u8; 32] = [0; 32];
        sha.input_str(blob);
        sha.result(&mut sha_result);

        let message: Message = Message::from_slice(&sha_result)?;
        let signature = secp.sign(&message, &priv_k);

        //eprintln!("SIG:{}", hex::encode(&signature.serialize_compact()));
        let sig: StdSignature = StdSignature::create(&signature.serialize_compact(), pub_k);
        Ok(sig)
    }
    /// used for testing
    /// could potentially be used to recreate the private key instead of words
    #[allow(dead_code)]
    pub(crate) fn seed(&self, passwd: &str) -> Option<Seed> {
        match &self.mnemonic {
            Some(phrase) => Some(phrase.to_seed(passwd)),
            None => None,
        }
    }
}

#[cfg(test)]
mod tst {
    use super::*;

    #[test]
    pub fn tst_gen_mnemonic() -> Result<()> {
        // this test just makes sure the default will call it.
        let s = Secp256k1::new();
        PrivateKey::new(&s).and_then(|_| Ok(()))
    }
    #[test]
    pub fn tst_words() -> Result<()> {
        let str_1 = "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius";
        let seed_1 = "a2ae8846397b55d266af35acdbb18ba1d005f7ddbdd4ca7a804df83352eaf373f274ba0dc8ac1b2b25f19dfcb7fa8b30a240d2c6039d88963defc2f626003b2f";
        let s = Secp256k1::new();
        let pk = PrivateKey::from_words(&s, str_1)?;
        assert_eq!(hex::encode(pk.seed("").unwrap().as_bytes()), seed_1);
        match pk.words() {
            Some(words) => {
                assert_eq!(words, str_1);
                Ok(())
            }
            None => Err("missing phrase".into()),
        }
    }
    #[test]
    pub fn tst_root_priv_key() -> Result<()> {
        let str_1 = "wonder caution square unveil april art add hover spend smile proud admit modify old copper throw crew happy nature luggage reopen exhibit ordinary napkin";
        let secp = Secp256k1::new();
        let pk = PrivateKey::from_words(&secp, str_1)?;
        let root_key = "xprv9s21ZrQH143K2ep3BpYRRMjSqjLHZAPAzxfVVS3NBuGKBVtCrK3C8mE8TcmTjYnLm7SJxdLigDFWGAMnctKxc3p5QKNWXdprcFSQzGzQqTW";
        assert_eq!(pk.root_private_key.to_string(), root_key);

        let derived_key = "4804e2bdce36d413206ccf47cc4c64db2eff924e7cc9e90339fa7579d2bd9d5b";
        assert_eq!(pk.private_key.private_key.key.to_string(), derived_key);

        Ok(())
    }
    #[test]
    pub fn tst_words_to_pub() -> Result<()> {
        let str_1 = "wonder caution square unveil april art add hover spend smile proud admit modify old copper throw crew happy nature luggage reopen exhibit ordinary napkin";
        let secp = Secp256k1::new();
        let pk = PrivateKey::from_words(&secp, str_1)?;
        let pub_k = pk.public_key(&secp);

        let account = pub_k.account()?;
        assert_eq!(&account, "terra1jnzv225hwl3uxc5wtnlgr8mwy6nlt0vztv3qqm");
        assert_eq!(
            &pub_k.TerraValOperPub()?,
            "terravaloperpub1addwnpepqt8ha594svjn3nvfk4ggfn5n8xd3sm3cz6ztxyugwcuqzsuuhhfq5y7accr"
        );
        assert_eq!(
            &pub_k.TerraPub()?,
            "terrapub1addwnpepqt8ha594svjn3nvfk4ggfn5n8xd3sm3cz6ztxyugwcuqzsuuhhfq5nwzrf9"
        );

        Ok(())
    }
    #[test]
    pub fn test_sign() -> Result<()> {
        // This test is using message from python SDK.. so these keys generate same sigs as they do.
        let str_1 =  "island relax shop such yellow opinion find know caught erode blue dolphin behind coach tattoo light focus snake common size analyst imitate employ walnut";
        let secp = Secp256k1::new();
        let pk = PrivateKey::from_words(&secp, str_1)?;
        let _pub_k = pk.public_key(&secp);
        let to_sign = r#"{"account_number":"45","chain_id":"columbus-3-testnet","fee":{"amount":[{"amount":"698","denom":"uluna"}],"gas":"46467"},"memo":"","msgs":[{"type":"bank/MsgSend","value":{"amount":[{"amount":"100000000","denom":"uluna"}],"from_address":"terra1n3g37dsdlv7ryqftlkef8mhgqj4ny7p8v78lg7","to_address":"terra1wg2mlrxdmnnkkykgqg4znky86nyrtc45q336yv"}}],"sequence":"0"}"#;

        let sig = pk.sign(&secp, to_sign)?;

        assert_eq!(
            sig.pub_key.value,
            "AiMzHaA2bvnDXfHzkjMM+vkSE/p0ymBtAFKUnUtQAeXe"
        );
        assert_eq!(sig.signature, "FJKAXRxNB5ruqukhVqZf3S/muZEUmZD10fVmWycdVIxVWiCXXFsUy2VY2jINEOUGNwfrqEZsT2dUfAvWj8obLg==");

        Ok(())
    }
}
