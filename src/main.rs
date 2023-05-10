// #0: derivation path
// #1: account address, max_fee, chain_id, nonce, version
// #2: call_array_len, calldata_len
// #n: {to, entrypoint_length, entrypoint, data_offset, data_len}
// #p: {calldata_0, calldata_1, ....}

//! Utility to generate APDU for Tx blur signing with Starknet Nano application
//! (see [Starknet Tx format](https://docs.starknet.io/documentation/architecture_and_concepts/Blocks/transactions/#invoke_transaction_version_1))

use apdu_generator::apdu::Call;

/// Derivation path
const PATH: &str = "m/2645'/1195502025'/1148870696'/0'/0'/0";
/// Hash
//const HASH: &str = "0x55b8f28706a5008d3103bcb2bfa6356e56b95c34fed265c955846670a6bb4ef";
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
const CALLS: [Call; 3] = [
    Call {
        to: "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        entrypoint: "transfer",
        selector: "0x83afd3f4caedc6eebf44246fe54e38c95e3179a5ec9ea81740eca5b482d12e",
        calldata: &[
            "0x07e00d496e324876bbc8531f2d9a82bf154d1a04a50218ee74cdd372f75a551a",
            "1000",
        ],
    },
    Call {
        to: "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        entrypoint: "transfer",
        selector: "0x83afd3f4caedc6eebf44246fe54e38c95e3179a5ec9ea81740eca5b482d12e",
        calldata: &[
            "0x07e00d496e324876bbc8531f2d9a82bf154d1a04a50218ee74cdd372f75a551a",
            "1000",
        ],
    },
    Call {
        to: "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        entrypoint: "transfer",
        selector: "0x83afd3f4caedc6eebf44246fe54e38c95e3179a5ec9ea81740eca5b482d12e",
        calldata: &[
            "0x07e00d496e324876bbc8531f2d9a82bf154d1a04a50218ee74cdd372f75a551a",
            "1000",
        ],
    },
];

fn main() {
    println!("=> Clear Sign Tx APDUs");
    match apdu_generator::builder::get_clear_sign_tx_apdu(PATH, &CALLS[..], AA, MAX_FEE, CHAIN_ID, NONCE, VERSION) {
        Ok(v) => {
            for apdu in v {
                println!("=> {apdu}\n")
            }
        }
        Err(_e) => println!("Internal error")
    }

    println!("=> Blur Sign Tx APDUs");
    match apdu_generator::builder::get_blur_sign_tx_apdu(PATH, &CALLS[..], AA, MAX_FEE, CHAIN_ID, NONCE, VERSION) {
        Ok(v) => {
            for apdu in v {
                println!("=> {apdu}\n")
            }
        }
        Err(_e) => println!("Internal error")
    }
    
}
