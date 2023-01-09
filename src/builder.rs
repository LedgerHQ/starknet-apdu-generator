use crate::apdu::{Apdu, CallArray, FieldElement, Ins};
use ethereum_types::U256;

pub fn build_path_apdu(ins: Ins, path: &str) -> Apdu {
    let mut apdu = Apdu::new();
    apdu.ins = ins;

    // APDU 0: derivation path
    apdu.p1 = 0x00;
    apdu.p2 = 0x00;
    let mut bip32_path: Vec<u32> = Vec::new();
    match path.strip_prefix("m/") {
        Some(spath) => {
            for s in spath.split('/') {
                let val: u32 = match s.ends_with('\'') {
                    true => 0x80000000 + s.strip_suffix('\'').unwrap().parse::<u32>().unwrap(),
                    false => s.parse::<u32>().unwrap(),
                };
                bip32_path.push(val);
            }
            for val in bip32_path {
                apdu.append(val.to_be_bytes().as_slice()).unwrap();
            }
            apdu
        }
        None => apdu,
    }
}

pub fn build_metadata_apdu(
    ins: Ins,
    aa: &str,
    max_fee: &str,
    chain_id: &str,
    nonce: &str,
    version: &str,
) -> Apdu {
    let mut apdu = Apdu::new();
    apdu.ins = ins;
    apdu.p1 = 0x01;
    apdu.p2 = 0x00;

    let sender_address: FieldElement = FieldElement(U256::from_str_radix(aa, 16).unwrap());
    let mut data: [u8; 32] = sender_address.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    let max_fee: FieldElement = FieldElement(U256::from_str_radix(max_fee, 10).unwrap());
    data = max_fee.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    let chain_id: FieldElement = FieldElement(U256::from_str_radix(chain_id, 16).unwrap());
    data = chain_id.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    let nonce: FieldElement = FieldElement(U256::from_dec_str(nonce).unwrap());
    data = nonce.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    let version: FieldElement = FieldElement(U256::from_dec_str(version).unwrap());
    data = version.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    apdu
}

pub fn build_calldata_apdu(ins: Ins, calls: &[CallArray]) -> Vec<Apdu> {
    let mut v: Vec<Apdu> = Vec::new();

    let mut apdu = Apdu::new();

    // APDU 2: call_array_len, calldata_len
    apdu.ins = ins;
    apdu.p1 = 0x02;
    let call_array_len: FieldElement =
        FieldElement(U256::from_big_endian(calls.len().to_be_bytes().as_slice()));
    let mut data: [u8; 32] = call_array_len.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    let mut total_calldata_len: u8 = 0;
    for c in calls.iter() {
        total_calldata_len += c.calldata.len() as u8;
    }
    let calldata_len: FieldElement = FieldElement(U256::from_big_endian(
        total_calldata_len.to_be_bytes().as_slice(),
    ));
    data = calldata_len.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();
    v.push(apdu);

    // APDUs CallArray
    let mut offset: u8 = 0;
    for (pos, c) in calls.iter().enumerate() {
        let mut apdu = Apdu::new();
        apdu.ins = ins;
        apdu.p1 = 0x03;
        apdu.p2 = pos as u8;

        let to: FieldElement = FieldElement(U256::from_str_radix(c.to, 16).unwrap());
        data = to.try_into().unwrap();
        apdu.append(data.as_slice()).unwrap();

        let entrypoint_length: u8 = c.entrypoint.len() as u8;
        apdu.append(&[entrypoint_length]).unwrap();
        apdu.append(c.entrypoint.as_bytes()).unwrap();

        let data_offset: FieldElement =
            FieldElement(U256::from_big_endian(offset.to_be_bytes().as_slice()));
        data = data_offset.try_into().unwrap();
        apdu.append(data.as_slice()).unwrap();

        let data_len: FieldElement = FieldElement(U256::from_big_endian(
            c.calldata.len().to_be_bytes().as_slice(),
        ));
        data = data_len.try_into().unwrap();
        apdu.append(data.as_slice()).unwrap();

        offset += c.calldata.len() as u8;
        v.push(apdu);
    }

    // APDUs Calldata
    for (pos, c) in calls.iter().enumerate() {
        let mut apdu = Apdu::new();
        apdu.ins = ins;
        apdu.p1 = 0x04;
        apdu.p2 = pos as u8;

        for item in c.calldata {
            let fe: FieldElement = match item.starts_with("0x") {
                false => FieldElement(U256::from_dec_str(item).unwrap()),
                true => FieldElement(U256::from_str_radix(item, 16).unwrap()),
            };
            data = fe.try_into().unwrap();
            apdu.append(data.as_slice()).unwrap();
        }
        v.push(apdu);
    }
    v
}
