use crate::errors::{ErrorKind, Result};
use bech32::{decode, encode, ToBase32, Variant};

fn check_prefix_and_length(prefix: &str, data: &str, length: usize) -> Result<bool> {
    let (hrp, _decoded, variant) = decode(data)?;
    if variant == Variant::Bech32 {
        Ok(hrp == prefix && data.len() == length)
    } else {
        Err(ErrorKind::Bech32DecodeErr.into())
    }
}
/***
Account Address
*/
pub struct AccAddress {
    pub data: String,
}

impl AccAddress {
    pub fn new(data: &str) -> AccAddress {
        AccAddress {
            data: data.to_owned(),
        }
    }
    pub fn validate(&self) -> Result<bool> {
        check_prefix_and_length("terra", &self.data, 44)
    }
    pub fn from_hex(key: &[u8]) -> Result<AccAddress> {
        let data = encode("terra", key.to_base32(), Variant::Bech32)?;
        Ok(AccAddress { data })
    }

    pub fn from_val_address(address: ValAddress) -> Result<AccAddress> {
        let (_hrp, decoded, variant) = decode(&address.data)?;
        let data = encode("terra", decoded, variant)?;
        Ok(AccAddress { data })
    }
}
/**
Validator Address
*/
pub struct ValAddress {
    pub data: String,
}
impl ValAddress {
    pub fn new(data: &str) -> ValAddress {
        ValAddress {
            data: data.to_owned(),
        }
    }
    pub fn validate(&self) -> Result<bool> {
        check_prefix_and_length("terravaloper", &self.data, 51)
    }
    pub fn from_hex(key: &[u8]) -> Result<ValAddress> {
        let data = encode("terravaloper", key.to_base32(), Variant::Bech32)?;
        Ok(ValAddress { data })
    }
    pub fn from_acc_address(address: AccAddress) -> Result<ValAddress> {
        let (_hrp, decoded, variant) = decode(&address.data)?;
        let data = encode("terravaloper", decoded, variant)?;
        Ok(ValAddress { data })
    }
}
/*
Validator Consensus Address
 */
pub struct ValConsAddress {
    pub data: String,
}

impl ValConsAddress {
    pub fn new(data: &str) -> ValConsAddress {
        ValConsAddress {
            data: data.to_owned(),
        }
    }
    pub fn validate(&self) -> Result<bool> {
        check_prefix_and_length("terravalcons", &self.data, 51)
    }
    pub fn from_hex(key: &[u8]) -> Result<ValConsAddress> {
        let data = encode("terravalcons", key.to_base32(), Variant::Bech32)?;
        Ok(ValConsAddress { data })
    }
}
/*
Account Pub Key
 */
pub struct AccPubKey {
    pub data: String,
}
impl AccPubKey {
    pub fn new(data: &str) -> AccPubKey {
        AccPubKey {
            data: data.to_owned(),
        }
    }

    pub fn from_hex(key: &[u8]) -> Result<AccPubKey> {
        let data = encode("terrapub", key.to_base32(), Variant::Bech32)?;
        Ok(AccPubKey { data })
    }

    pub fn validate(&self) -> Result<bool> {
        check_prefix_and_length("terrapub", &self.data, 76)
    }
    pub fn from_val_pubkey(pubkey: ValPubKey) -> Result<AccPubKey> {
        let (_hrp, decoded, variant) = decode(&pubkey.data)?;
        let data = encode("terrapub", decoded, variant)?;
        Ok(AccPubKey { data })
    }
}
/*
  Validator Pub Key
*/
pub struct ValPubKey {
    pub data: String,
}
impl ValPubKey {
    pub fn new(data: &str) -> ValPubKey {
        ValPubKey {
            data: data.to_owned(),
        }
    }
    pub fn validate(&self) -> Result<bool> {
        check_prefix_and_length("terravaloperpub", &self.data, 83)
    }
    pub fn from_acc_pubkey(pubkey: AccPubKey) -> Result<ValPubKey> {
        let (_hrp, decoded, variant) = decode(&pubkey.data)?;
        let data = encode("terravaloperpub", decoded, variant)?;
        Ok(ValPubKey { data })
    }
    pub fn from_hex(key: &[u8]) -> Result<ValPubKey> {
        let data = encode("terravaloperpub", key.to_base32(), Variant::Bech32)?;
        Ok(ValPubKey { data })
    }
}
/*
  Validator Pub Key
*/
pub struct ValConsPubKey {
    pub data: String,
}
impl ValConsPubKey {
    pub fn new(data: &str) -> ValConsPubKey {
        ValConsPubKey {
            data: data.to_owned(),
        }
    }
    pub fn validate(&self) -> Result<bool> {
        check_prefix_and_length("terravalconspub", &self.data, 82)
    }
    pub fn from_hex(key: &[u8]) -> Result<ValConsPubKey> {
        let data = encode("terravalconspub", key.to_base32(), Variant::Bech32)?;
        Ok(ValConsPubKey { data })
    }
}

/**
* Gets a raw address from a compressed bytes public key.
*
* @param publicKey raw public key

*/

fn address_from_public_key(public_key: &[u8]) -> Result<Vec<bech32::u5>> {
    use ripemd160::{Digest, Ripemd160};

    let sha = sha256::digest_bytes(public_key);
    let mut hasher = Ripemd160::new();
    hasher.update(sha);
    let address: Vec<u8> = hasher.finalize().to_owned().to_vec();

    //const message = HEX.parse(publicKey.toString('hex'));
    //const hash = RIPEMD160(SHA256(message) as any).toString();
    //const address = Buffer.from(hash, 'hex');
    //return Buffer.from(bech32.toWords(address));
    let words = address.to_base32();
    Ok(words)
    // let encoded = bech32::encode("terrapub", address, Variant::Bech32);
}

#[cfg(test)]
mod tst {
    use super::*;
    #[test]
    pub fn tst_val_cons_pub() {
        let v1 = ValConsPubKey::new(
            "terravalconspub1zcjduepq83p63k33qgj3q7z00swxwmr9dyjv5p905fk74k5r7lkpdg9xqleqljhrtt",
        )
        .validate();

        assert!(v1.is_ok());
        assert!(v1.unwrap());
        let v2 =
            ValConsPubKey::new("terravaloper1usws7c2c6cs7nuc8vma9qzaky5pkgvm2ujy8ny").validate();
        assert!(v2.is_ok());
        assert!(!v2.unwrap());
        let v3 =
            ValConsPubKey::new("terravaloper1usws7c2c6cs7nuc8vma9qzaky5pkgvm2ujyny").validate();
        assert!(!v3.is_ok());
        let v4 = ValConsPubKey::new(
            "terravalconspub1zcjduepq83p63k33qgj3q7z00swxwmr9dyjv5p905fk74k5r7lkpdg9xqleqZZZZZZ",
        )
        .validate();
        assert!(!v4.is_ok());
    }
    #[test]
    pub fn tst_valoper_pub() {
        let v1 = ValAddress::new("terravaloper12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69nyeu9q");
        let v1v = v1.validate();

        assert!(v1v.is_ok());
        assert!(v1v.unwrap());
        let acc1 = AccAddress::new("terra1824vxwh43h9d3qczj4jvc3qphlf2evfp9w0ph9");
        assert!(acc1.validate().unwrap());
        let a1 = ValAddress::from_acc_address(acc1);
        assert!(a1.is_ok());
        let v2 = a1.unwrap();
        eprintln!("{:#?}", v2.data);
    }
    #[test]
    pub fn from_pubkey() -> Result<()> {
        let pubkey: [u8; 1] = [0x0];
        let acc_pubkey = AccPubKey::from_hex(&pubkey)?;
        assert_eq!(acc_pubkey.data, "terrapub1qquzhzzn");
        let address = AccAddress::from_hex(&pubkey)?;
        assert_eq!(address.data, "terra1qq2zjtpc");
        assert_eq!(ValAddress::from_hex(&pubkey)?.data, "terravaloper1qqghypy5");
        assert_eq!(
            ValPubKey::from_hex(&pubkey)?.data,
            "terravaloperpub1qqyseyxc"
        );
        assert_eq!(
            ValConsAddress::from_hex(&pubkey)?.data,
            "terravalcons1qqg8r6gt"
        );
        assert_eq!(
            ValConsPubKey::from_hex(&pubkey)?.data,
            "terravalconspub1qqcsh2nx"
        );
        Ok(())
    }
}
