// #0: derivation path
// #1: account address, max_fee, chain_id, nonce, version
// #2: call_array_len, calldata_len
// #n: {to, entrypoint_length, entrypoint, data_offset, data_len}
// #p: {calldata_0, calldata_1, ....}

mod apdu;
mod builder;
use apdu::{CallArray, Ins};
use builder::{build_calldata_apdu, build_metadata_apdu, build_path_apdu};

const PATH: &str = "m/2645'/1195502025'/1148870696'/0'/0'/0";
const AA: &str = "0x07e00d496e324876bbc8531f2d9a82bf154d1a04a50218ee74cdd372f75a551a";
const MAX_FEE: &str = "1000000000000000";
const CHAIN_ID: &str = "534e5f474f45524c49";
const NONCE: &str = "1";
const VERSION: &str = "1";

const CALL_ARRAYS: [CallArray; 1] = [
    CallArray {
        to: "0x0507446de5cfcb833d4e786f3a0510deb2429ae753741a836a7efa80c9c747cb",
        entrypoint: "mint",
        calldata: [
            "0x07e00d496e324876bbc8531f2d9a82bf154d1a04a50218ee74cdd372f75a551a",
            "1000",
        ],
    },
    /*CallArray {
        to: "0x0507446de5cfcb833d4e786f3a0510deb2429ae753741a836a7efa80c9c747cb",
        entrypoint: "mint",
        calldata: ["0x07e00d496e324876bbc8531f2d9a82bf154d1a04a50218ee74cdd372f75a551a", "1000"]
    },*/
];

fn main() {
    println!("=> {}", build_path_apdu(Ins::SignTx, PATH));

    println!(
        "=> {}",
        build_metadata_apdu(Ins::SignTx, AA, MAX_FEE, CHAIN_ID, NONCE, VERSION)
    );

    for s in build_calldata_apdu(Ins::SignTx, &CALL_ARRAYS[..]) {
        println!("=> {}", s);
    }
}
