// #0: derivation path
// #1: account address, max_fee, chain_id, nonce, version
// #2: call_array_len, calldata_len
// #n: {to, entrypoint_length, entrypoint, data_offset, data_len}
// #p: {calldata_0, calldata_1, ....}

//! Utility to generate APDU for Tx blur signing with Starknet Nano application
//! (see [Starknet Tx format](https://docs.starknet.io/documentation/architecture_and_concepts/Blocks/transactions/#invoke_transaction_version_1))

use apdu_generator::apdu::Call;
use apdu_generator::builder::{
    get_pubkey_apdu, get_sign_hash_apdu, get_sign_tx_apdu, get_version_apdu,
};

/// Derivation path
const PATH: &str = "m/2645'/1195502025'/1148870696'/0'/0'/0";
/// Hash
const HASH: &str = "0x55b8f28706a5008d3103bcb2bfa6356e56b95c34fed265c955846670a6bb4ef";
/// Account contract address
const AA: &str = "0x07e00d496e324876bbc8531f2d9a82bf154d1a04a50218ee74cdd372f75a551a";
/// Max Fee
const MAX_FEE: &str = "1000000000000000";
/// Chain ID
const CHAIN_ID: &str = "534e5f474f45524c49";
/// Nonce
const NONCE: &str = "1";
/// Version number
const VERSION: &str = "1";
/// Calls
const CALLS: [Call; 2] = [
    Call {
        to: "0x0507446de5cfcb833d4e786f3a0510deb2429ae753741a836a7efa80c9c747cb",
        entrypoint: "mint",
        calldata: [
            "0x07e00d496e324876bbc8531f2d9a82bf154d1a04a50218ee74cdd372f75a551a",
            "1000",
        ],
    },
    Call {
        to: "0x0507446de5cfcb833d4e786f3a0510deb2429ae753741a836a7efa80c9c747cb",
        entrypoint: "approve",
        calldata: [
            "0x07e00d496e324876bbc8531f2d9a82bf154d1a04a50218ee74cdd372f75a551a",
            "10000",
        ],
    },
];

fn main() {
    println!("=> Get Version APDUs");
    match get_version_apdu() {
        Ok(v) => {
            for apdu in v {
                println!("=> {apdu}");
            }
        }
        Err(_e) => println!("Internal error")
    }
    

    println!("=> Get Pub key APDUs");
    match get_pubkey_apdu(PATH) {
        Ok(v) => {
            for apdu in v {
                println!("=> {apdu}");
            }
        }
        Err(_e) => println!("Internal error")
    }
    

    println!("=> Sign Hash APDUs");
    match get_sign_hash_apdu(PATH, HASH, true) {
        Ok(v) => {
            for apdu in v {
                println!("=> {apdu}")
            }
        }
        Err(_e) => println!("Internal error")
    }
    

    println!("=> Sign Tx APDUs");
    match get_sign_tx_apdu(PATH, &CALLS[..], AA, MAX_FEE, CHAIN_ID, NONCE, VERSION) {
        Ok(v) => {
            for apdu in v {
                println!("=> {apdu}")
            }
        }
        Err(_e) => println!("Internal error")
    }
    
}
