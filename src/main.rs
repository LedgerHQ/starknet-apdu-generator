//! Utility to generate APDU for Tx blur or clear signing with Starknet Nano application
//! (see [Starknet Tx format](https://docs.starknet.io/documentation/architecture_and_concepts/Blocks/transactions/#invoke_transaction_version_1))

/// Derivation path
const PATH: &str = "m/2645'/1195502025'/1148870696'/0'/0'/0";
/// Hash
//const HASH: &str = "0x55b8f28706a5008d3103bcb2bfa6356e56b95c34fed265c955846670a6bb4ef";

use std::fs::File;
use std::io::prelude::*;

fn main() {
    
    let mut file = File::open("transaction.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let tx: apdu_generator::types::Tx = serde_json::from_str(&data).unwrap();

    println!("=> Clear Sign Tx APDUs:");
    match apdu_generator::builder::get_clear_sign_apdus(PATH, &tx.calls, &tx.sender_address, &tx.max_fee, &tx.chain_id, &tx.nonce, &tx.version) {
        Ok(v) => {
            for apdu in v {
                println!("{apdu}")
            }
        }
        Err(_e) => println!("Internal error")
    }

}
