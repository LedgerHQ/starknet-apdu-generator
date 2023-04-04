use ethereum_types::U256;
use std::fmt;
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

//const MAX_APDU_SIZE: usize = 260;
const MAX_APDU_DATA_SIZE: usize = 255;

pub struct ApduHeader {
    pub cla: u8,
    pub ins: u8,
    pub p1: u8,
    pub p2: u8,
}

pub struct Apdu {
    pub header: ApduHeader,
    pub len: usize,
    pub data: [u8; MAX_APDU_DATA_SIZE],
}

impl Apdu {
    pub fn new(cla: u8, ins: u8, p1: u8, p2: u8) -> Self {
        Apdu {
            header: ApduHeader {
                cla,
                ins,
                p1,
                p2,
            },
            len: 0x00,
            data: [0u8; MAX_APDU_DATA_SIZE],
        }
    }

    pub fn append(&mut self, data: &[u8]) -> Result<(), usize> {
        if self.len + data.len() <= MAX_APDU_DATA_SIZE {
            self.data[self.len..self.len + data.len()].copy_from_slice(data);
            self.len += data.len();
            Ok(())
        } else {
            Err(MAX_APDU_DATA_SIZE - self.len)
        }
    }
}

impl fmt::Display for Apdu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02x}", self.header.cla)?;
        let ins: u8 = self.header.ins;
        write!(f, "{:02x}", ins)?;
        write!(f, "{:02x}", self.header.p1)?;
        write!(f, "{:02x}", self.header.p2)?;
        write!(f, "{:02x}", self.len)?;
        for b in 0..self.len {
            write!(f, "{:02x}", self.data[b])?;
        }
        Ok(())
    }
}

pub struct Call<'a> {
    pub to: &'a str,
    pub entrypoint: &'a str,
    pub calldata: [&'a str; 2],
}
