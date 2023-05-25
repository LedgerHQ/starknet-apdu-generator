use ethereum_types::U256;
use std::fmt;
use serde::Deserialize;

pub struct FieldElement(pub U256);
impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = [0u8; 32];
        self.0.to_big_endian(&mut s[..]);
        for b in s {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}

impl TryFrom<FieldElement> for [u8; 32] {
    type Error = ();
    fn try_from(fe: FieldElement) -> Result<Self, Self::Error> {
        let mut s = [0u8; 32];
        fe.0.to_big_endian(&mut s[..]);
        Ok(s)
    }
}

#[derive(Copy, Clone)]
pub enum Ins {
    GetVersion,
    GetPubkey,
    SignHash,
    SignTx,
    PedersenHash,
}

impl TryFrom<Ins> for u8 {
    type Error = ();
    fn try_from(value: Ins) -> Result<Self, Self::Error> {
        match value {
            Ins::GetVersion => Ok(0),
            Ins::GetPubkey => Ok(1),
            Ins::SignHash => Ok(2),
            Ins::SignTx => Ok(3),
            Ins::PedersenHash => Ok(4),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Call {
    pub to: String,
    pub entrypoint: String,
    pub selector: String,
    pub calldata: Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct Tx {
    pub sender_address: String,
    pub max_fee: String,
    pub nonce: String,
    pub version: String,
    pub chain_id: String,
    pub calls: Vec<Call>
}