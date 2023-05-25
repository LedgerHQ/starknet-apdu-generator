use std::fmt;

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

    pub fn clear(&mut self) {
        self.data.fill(0);
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
