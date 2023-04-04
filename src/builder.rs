use crate::apdu::{Apdu, Call, FieldElement};
use ethereum_types::U256;

mod builder_internal;
use builder_internal::{
    build_callarray_apdu, build_calldata_apdu, build_calls_metadata, build_set_derivation_path,
    build_tx_metadata, fix,
};

pub enum ApduError {
    InternalError
}

pub fn get_version_apdu() -> Result<Vec<Apdu>, ApduError> {
    Ok(vec![Apdu::new(0x80, 0x00, 0x00, 0x00)])
}

pub fn get_pubkey_apdu(path: &str) -> Result<Vec<Apdu>, ApduError> {
    let mut apdu = Apdu::new(0x80, 0x01, 0x00, 0x00);
    build_set_derivation_path(path, &mut apdu);
    Ok(vec![apdu])
}

pub fn get_sign_hash_apdu(path: &str, hash: &str, show_hash: bool) -> Result<Vec<Apdu>, ApduError> {
    let mut v: Vec<Apdu> = Vec::new();

    let mut apdu = Apdu::new(0x80, 0x02, 0x00, 0x00);
    build_set_derivation_path(path, &mut apdu);
    v.push(apdu);

    apdu = Apdu::new(
        0x80,
        0x02,
        0x00,
        match show_hash {
            true => 0x01,
            false => 0x00,
        },
    );

    //let mut fixed_hash = String::from(hash);
    let mut fixed_hash = String::from(hash.trim_start_matches("0x"));
    fix(&mut fixed_hash);
    let data: [u8; 32] = FieldElement(U256::from_str_radix(fixed_hash.as_str(), 16).unwrap())
        .try_into()
        .unwrap();
    match apdu.append(&data[..]) {
        Ok(()) => v.push(apdu),
        Err(_e) => {
            return Err(ApduError::InternalError);
        }
    }

    Ok(v)
}

pub fn get_sign_tx_apdu(
    path: &str,
    calls: &[Call],
    aa: &str,
    max_fee: &str,
    chain_id: &str,
    nonce: &str,
    version: &str,
) -> Result<Vec<Apdu>, ApduError> {
    let mut v: Vec<Apdu> = Vec::new();

    // Derivation path
    let mut apdu = Apdu::new(0x80, 0x03, 0x00, 0x00);
    build_set_derivation_path(path, &mut apdu);
    v.push(apdu);

    // Tx metadata: account address, max_fee, chain_id, nonce, version
    apdu = Apdu::new(0x80, 0x03, 0x01, 0x00);
    build_tx_metadata(aa, max_fee, chain_id, nonce, version, &mut apdu);
    v.push(apdu);

    // Calls metadata: call_array length and calldata length
    apdu = Apdu::new(0x80, 0x03, 0x02, 0x00);
    build_calls_metadata(calls, &mut apdu);
    v.push(apdu);

    // For every single call, 2 APDUs will be provided:
    //  - call.metadata (to, entry_point/selector, data_offset, data_length)
    //  - call.calldata
    // !! all call.metadata APDUs are provided then all call.calldata APDUs (pedersen hash calcultation optimization)
    let mut offset: u8 = 0;
    let mut temp: Vec<Apdu> = vec![];
    for (pos, c) in calls.iter().enumerate() {
        apdu = Apdu::new(0x80, 0x03, 0x03, pos as u8);
        build_callarray_apdu(c, &mut apdu, &offset);
        v.push(apdu);

        apdu = Apdu::new(0x80, 0x03, 0x04, pos as u8);
        build_calldata_apdu(c, &mut apdu);
        temp.push(apdu);

        offset += c.calldata.len() as u8;
    }
    v.append(&mut temp);

    Ok(v)
}
