use crate::errors::{ErrorKind, Result};

use bitcoin::bech32::{decode, encode, u5, FromBase32, ToBase32};
use crypto::digest::Digest;
use crypto::ripemd160::Ripemd160;
use crypto::sha2::Sha256;
use serde::{Deserialize, Serialize};

static BECH32_PUBKEY_DATA_PREFIX: [u8; 5] = [0xeb, 0x5a, 0xe9, 0x87, 0x21]; // "eb5ae98721";

#[derive(Deserialize, Serialize, Debug)]

pub struct PublicKey {
    pub raw_pub_key: Option<Vec<u8>>,
    pub raw_address: Option<Vec<u8>>,
}

impl PublicKey {
    pub fn from_bitcoin_public_key(bpub: &bitcoin::util::key::PublicKey) -> PublicKey {
        let bpub_bytes = bpub.key.serialize();
        //     eprintln!("B-PK-{}", hex::encode(bpub_bytes));
        let raw_pub_key = PublicKey::pubkey_from_public_key(&bpub_bytes);
        let raw_address = PublicKey::address_from_public_key(&bpub_bytes);

        PublicKey {
            raw_pub_key: Some(raw_pub_key),
            raw_address: Some(raw_address),
        }
    }
    pub fn from_public_key(bpub: &[u8]) -> PublicKey {
        let raw_pub_key = PublicKey::pubkey_from_public_key(bpub);
        let raw_address = PublicKey::address_from_public_key(bpub);

        PublicKey {
            raw_pub_key: Some(raw_pub_key),
            raw_address: Some(raw_address),
        }
    }
    pub fn from_account(acc_address: &str) -> Result<PublicKey> {
        PublicKey::check_prefix_and_length("terra", acc_address, 44).and_then(|vu5| {
            match Vec::from_base32(vu5.as_slice()) {
                Ok(vu8) => Ok(PublicKey {
                    raw_pub_key: None,
                    raw_address: Some(vu8),
                }),
                Err(_) => Err(ErrorKind::Conversion(String::from(acc_address)).into()),
            }
        })
    }
    pub fn from_operator_address(valoper_address: &str) -> Result<PublicKey> {
        PublicKey::check_prefix_and_length("terravaloper", valoper_address, 51).and_then(|vu5| {
            match Vec::from_base32(vu5.as_slice()) {
                Ok(vu8) => Ok(PublicKey {
                    raw_pub_key: None,
                    raw_address: Some(vu8),
                }),
                Err(_) => Err(ErrorKind::Conversion(String::from(valoper_address)).into()),
            }
        })
    }

    pub fn from_raw_address(raw_address: &str) -> Result<PublicKey> {
        // let bech32_prefix = hex::encode(BECH32_PUBKEY_DATA_PREFIX);
        //if raw_pub_key.starts_with(&bech32_prefix) {
        let vec1 = hex::decode(raw_address)?;
        //   let vec2 = &vec1.as_slice()[BECH32_PUBKEY_DATA_PREFIX.len()..];
        //   eprintln!("{}", hex::encode(&vec1));
        //   eprintln!("{}", hex::encode(vec2));
        //    let raw_address = PublicKey::address_from_public_key(&vec2);

        Ok(PublicKey {
            raw_pub_key: None,
            raw_address: Some(vec1),
        })
        //} else {
        //    Err(ErrorKind::Conversion(String::from(raw_pub_key)).into())
        // }
    }
    fn check_prefix_and_length(prefix: &str, data: &str, length: usize) -> Result<Vec<u5>> {
        match decode(data) {
            Ok((hrp, decoded_str)) => {
                if hrp == prefix && data.len() == length {
                    Ok(decoded_str)
                } else {
                    Err(ErrorKind::Bech32DecodeErr.into())
                }
            }
            Err(_) => Err(ErrorKind::Conversion(String::from(data)).into()),
        }
    }
    /**
     * Gets a bech32-words pubkey from a compressed bytes public key.
     *
     * @param publicKey raw public key
     */
    pub fn pubkey_from_public_key(public_key: &[u8]) -> Vec<u8> {
        //Vec<bech32::u5> {
        //   let mut buf = BECH32_PUBKEY_DATA_PREFIX.to_vec();
        //  buf.extend_from_slice(publicKey);
        [BECH32_PUBKEY_DATA_PREFIX.to_vec(), public_key.to_vec()].concat()
        //     .to_base32()
    }

    /**
    * Gets a raw address from a compressed bytes public key.
    *
    * @param publicKey raw public key

    */

    pub fn address_from_public_key(public_key: &[u8]) -> Vec<u8> {
        // Vec<bech32::u5> {

        let mut hasher = Ripemd160::new();
        let mut sha = Sha256::new();
        let mut sha_result: [u8; 32] = [0; 32];
        let mut ripe_result: [u8; 20] = [0; 20];

        sha.input(public_key);
        sha.result(&mut sha_result);
        //    eprintln!(".{}", encode_hex(&sha_result).unwrap());

        hasher.input(&sha_result);
        hasher.result(&mut ripe_result);

        // eprintln!("-{}", encode_hex(&ripe_result).unwrap());

        let address: Vec<u8> = ripe_result.to_vec();
        address
    }
    pub fn account(&self) -> Result<String> {
        match &self.raw_address {
            Some(raw) => {
                let data = encode("terra", raw.to_base32());
                match data {
                    Ok(acc) => Ok(acc),
                    Err(_) => Err(ErrorKind::Bech32DecodeErr.into()),
                }
            }
            None => Err(ErrorKind::Implementation.into()),
        }
    }
    pub fn operator_address(&self) -> Result<String> {
        match &self.raw_address {
            Some(raw) => {
                let data = encode("terravaloper", raw.to_base32());
                match data {
                    Ok(acc) => Ok(acc),
                    Err(_) => Err(ErrorKind::Bech32DecodeErr.into()),
                }
            }
            None => Err(ErrorKind::Implementation.into()),
        }
    }
    #[allow(non_snake_case)]
    pub fn TerraPub(&self) -> Result<String> {
        match &self.raw_pub_key {
            Some(raw) => {
                let data = encode("terrapub", raw.to_base32());
                match data {
                    Ok(acc) => Ok(acc),
                    Err(_) => Err(ErrorKind::Bech32DecodeErr.into()),
                }
            }
            None => Err(ErrorKind::Implementation.into()),
        }
    }
    #[allow(non_snake_case)]
    pub fn TerraValOperPub(&self) -> Result<String> {
        match &self.raw_pub_key {
            Some(raw) => {
                let data = encode("terravaloperpub", raw.to_base32());
                match data {
                    Ok(acc) => Ok(acc),
                    Err(_) => Err(ErrorKind::Bech32DecodeErr.into()),
                }
            }
            None => Err(ErrorKind::Implementation.into()),
        }
    }
    // TODO verify if this is dervied from 'raw_address' or 'raw_pubkey'
    #[allow(non_snake_case)]
    pub fn ValConsAddress(&self) -> Result<String> {
        match &self.raw_address {
            Some(raw) => {
                let data = encode("terravalcons", raw.to_base32());
                match data {
                    Ok(acc) => Ok(acc),
                    Err(_) => Err(ErrorKind::Bech32DecodeErr.into()),
                }
            }
            None => Err(ErrorKind::Implementation.into()),
        }
    }
    #[allow(non_snake_case)]
    pub fn ValConsPub(&self) -> Result<String> {
        match &self.raw_pub_key {
            Some(raw) => {
                let data = encode("terravalconspub", raw.to_base32());
                match data {
                    Ok(acc) => Ok(acc),
                    Err(_) => Err(ErrorKind::Bech32DecodeErr.into()),
                }
            }
            None => Err(ErrorKind::Implementation.into()),
        }
    }
}
#[cfg(test)]
mod tst {
    use super::*;
    #[test]
    pub fn tst_conv() -> Result<()> {
        let pub_key = PublicKey::from_account("terra1jnzv225hwl3uxc5wtnlgr8mwy6nlt0vztv3qqm")?;

        assert_eq!(
            &pub_key.account()?,
            "terra1jnzv225hwl3uxc5wtnlgr8mwy6nlt0vztv3qqm"
        );
        assert_eq!(
            &pub_key.operator_address()?,
            "terravaloper1jnzv225hwl3uxc5wtnlgr8mwy6nlt0vztraasg"
        );
        assert_eq!(
            &pub_key.ValConsAddress()?,
            "terravalcons1jnzv225hwl3uxc5wtnlgr8mwy6nlt0vzlswpuf"
        );
        let x = &pub_key.raw_address.unwrap();
        assert_eq!(hex::encode(x), "94c4c52a9777e3c3628e5cfe819f6e26a7f5bd82");

        Ok(())
    }
    #[test]
    pub fn test_pete() -> Result<()> {
        let pub_key = PublicKey::from_public_key(&hex::decode(
            "02cf7ed0b5832538cd89b55084ce93399b186e381684b31388763801439cbdd20a",
        )?);

        assert_eq!(
            &pub_key.operator_address()?,
            "terravaloper1jnzv225hwl3uxc5wtnlgr8mwy6nlt0vztraasg"
        );
        assert_eq!(
            &pub_key.account()?.to_string(),
            "terra1jnzv225hwl3uxc5wtnlgr8mwy6nlt0vztv3qqm"
        );
        assert_eq!(
            &pub_key.TerraPub()?,
            "terrapub1addwnpepqt8ha594svjn3nvfk4ggfn5n8xd3sm3cz6ztxyugwcuqzsuuhhfq5nwzrf9"
        );
        assert_eq!(
            &pub_key.ValConsPub()?,
            "terravalconspub1addwnpepqt8ha594svjn3nvfk4ggfn5n8xd3sm3cz6ztxyugwcuqzsuuhhfq5z3fguk"
        );

        let x = &pub_key.raw_address.unwrap();
        assert_eq!(hex::encode(x), "94c4c52a9777e3c3628e5cfe819f6e26a7f5bd82");
        let y = pub_key.raw_pub_key.unwrap();
        assert_eq!(
            hex::encode(y),
            "eb5ae9872102cf7ed0b5832538cd89b55084ce93399b186e381684b31388763801439cbdd20a"
        );
        //   eprintln!("{}", hex::encode(&pub_key.raw_pub_key.unwrap()));
        Ok(())
    }
}
