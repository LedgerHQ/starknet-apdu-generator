//! Utility to generate APDU for Tx blur or clear signing with Starknet Nano application
//! (see [Starknet Tx format](https://docs.starknet.io/documentation/architecture_and_concepts/Blocks/transactions/#invoke_transaction_version_1))

/// Derivation path
const PATH: &str = "m/2645'/1195502025'/1148870696'/0'/0'/0";
/// Hash
//const HASH: &str = "0x55b8f28706a5008d3103bcb2bfa6356e56b95c34fed265c955846670a6bb4ef";

use std::fs::File;
use std::io::prelude::*;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Tx in JSON format filename
    #[arg(short, long)]
    json: String,

    /// APDU CLA
    #[arg(short, long, default_value_t = 0x80)]
    cla: u8,

    /// APDU INS
    #[arg(short, long, default_value_t = 0x03)]
    ins: u8
}

use starknet_apdu_generator::{
    apdu::Apdu,
    types::Tx,
    builder
};

fn main() {

    let args: Args = Args::parse();
    
    let mut file = File::open(args.json).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let mut tx: Tx = serde_json::from_str(&data).unwrap();
    tx.calls.reverse();
    
    let mut apdus: Vec<Apdu> = Vec::new();

    let dpath_apdu = builder::derivation_path_to_apdu(PATH, args.cla, args.ins.into(), 0);
    apdus.push(dpath_apdu.clone());
    
    let txinfo_apdu = builder::txinfo_to_apdu(&tx, args.cla, args.ins.into(), 1);
    apdus.push(txinfo_apdu.clone());
    
    while tx.calls.len() > 0 {
        let call = tx.calls.pop().unwrap();
        let mut call_apdu = builder::call_to_apdu(&call, args.cla, args.ins.into());
        apdus.append(&mut call_apdu);
    }
    

    let mut json_out = File::create("apdu.json").unwrap();
    let mut raw_out = File::create("apdu.dat").unwrap();
    for a in apdus.iter() {
        println!("=> {}", a);
        writeln!(raw_out, "=> {}", a).unwrap();
    }
    writeln!(json_out, "{}", serde_json::to_string_pretty(&apdus).unwrap()).unwrap();
    
}
