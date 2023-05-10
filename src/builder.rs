use crate::apdu::{Apdu, Call, FieldElement};
use ethereum_types::U256;

mod builder_internal;
use builder_internal::{
    build_callarray_apdu, build_calldata_apdu, build_calls_metadata, build_callarray_len, build_set_derivation_path,
    build_tx_metadata, fix,
};

use self::builder_internal::{
    build_callarray_v1_apdu,
    build_calldata_v1_apdu
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

pub fn get_blur_sign_tx_apdu(
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

pub fn get_clear_sign_tx_apdu(
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
    let mut apdu = Apdu::new(0x80, 0x05, 0x00, 0x00);
    build_set_derivation_path(path, &mut apdu);
    v.push(apdu);

    // call_array length 
    apdu = Apdu::new(0x80, 0x05, 0x01, 0x00);
    build_callarray_len(calls, &mut apdu);
    v.push(apdu);

    for i in 0..calls.len() {
        // call_array 
        apdu = Apdu::new(0x80, 0x05, 0x02, i as u8);
        build_callarray_v1_apdu(&calls[i], &mut apdu);
        v.push(apdu);
        
        // call_data
        let len = calls[i].calldata.len();
        /* 7 is the max number of 32-byte FieldElement in an APDU: 7 * 32 = 224 bytes <= MAX_APDU_DATA_SIZE */
        let nb_calldata_apdu = len / 7;
        for j in 0..=nb_calldata_apdu {
            let cdata = &calls[i].calldata[j*7..std::cmp::min((j+1)*7, len)];
            apdu = Apdu::new(0x80, 0x05, 0x03, j as u8);
            build_calldata_v1_apdu(cdata, &mut apdu);
            v.push(apdu);
        }
    }

    // Tx metadata: account address, max_fee, chain_id, nonce, version
    apdu = Apdu::new(0x80, 0x03, 0x04, 0x00);
    build_tx_metadata(aa, max_fee, chain_id, nonce, version, &mut apdu);
    v.push(apdu);

    Ok(v)
}
