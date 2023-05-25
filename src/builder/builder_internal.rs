use crate::apdu::{Apdu, ApduHeader};
use crate::types::{Call, FieldElement};
use ethereum_types::U256;

/// Build Derivation path APDU
pub fn set_derivation_path_apdu(path: &str, apdu_header: ApduHeader) -> Apdu {

    let mut apdu = Apdu::new(apdu_header);

    let mut bip32_path: Vec<u32> = Vec::new();
    if let Some(spath) = path.strip_prefix("m/") {
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
    }
    apdu
}

pub fn callarray_len_apdu(calls: &[Call], apdu_header: ApduHeader) -> Apdu {

    let mut apdu = Apdu::new(apdu_header);

    let call_array_len = FieldElement(U256::from(calls.len()));
    let data: [u8; 32] = call_array_len.try_into().unwrap();
    apdu.append(&data[..]).unwrap();

    apdu
}

pub fn callarray_v1_apdu(c: &Call, apdu_header: ApduHeader) -> Apdu {
    
    let mut apdu = Apdu::new(apdu_header);

    let mut data: [u8; 32];

    let to: FieldElement = FieldElement(U256::from_str_radix(&c.to, 16).unwrap());
    data = to.try_into().unwrap();
    apdu.append(&data[..]).unwrap();

    let selector: FieldElement = FieldElement(U256::from_str_radix(&c.selector, 16).unwrap());
    data = selector.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    let call_data_len: FieldElement =
        FieldElement(U256::from(c.calldata.len()));
    data = call_data_len.try_into().unwrap();
    apdu.append(&data[..]).unwrap();

    apdu
}

pub fn calldata_v1_apdu(calldata: &[String], apdu_header: ApduHeader) -> Apdu {

    let mut apdu = Apdu::new(apdu_header);
    
    let mut data: [u8; 32];

    for cd in calldata {
        let d = FieldElement(U256::from_str_radix(cd, 16).unwrap());
        data = d.try_into().unwrap();
        apdu.append(data.as_slice()).unwrap();
    }

    apdu
}

/// Build Starknet Tx fields APDU
pub fn tx_metadata_apdu (
    sender_address: &str,
    max_fee: &str,
    chain_id: &str,
    nonce: &str,
    version: &str,
    apdu_header: ApduHeader,
) -> Apdu {

    let mut apdu = Apdu::new(apdu_header);

    let sender: FieldElement = FieldElement(U256::from_str_radix(sender_address, 16).unwrap());
    let mut data: [u8; 32] = sender.try_into().unwrap();
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

pub fn fix(hash: &mut String) {
    // fix pedersen hash to fit into 32 bytes
    while hash.len() < 63 {
        hash.insert(0, '0');
    }
    assert!(hash.len() == 63);
    hash.push('0');
}

/*pub fn build_calls_metadata(calls: &[Call], apdu: &mut Apdu) {
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

}*/

/*pub fn build_callarray_apdu(c: &Call, apdu: &mut Apdu, offset: &u8) {
    let mut data: [u8; 32];

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
}

/// Build Starknet Calldata APDU
pub fn build_calldata_apdu(c: &Call, apdu: &mut Apdu) {
    let mut data: [u8; 32];

    for item in c.calldata {
        let fe: FieldElement = match item.starts_with("0x") {
            false => FieldElement(U256::from_dec_str(item).unwrap()),
            true => FieldElement(U256::from_str_radix(item, 16).unwrap()),
        };
        data = fe.try_into().unwrap();
        apdu.append(data.as_slice()).unwrap();
    }
}*/