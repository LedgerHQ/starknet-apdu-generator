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
    ins: u8,

    /// fileout
    #[arg(short, long, default_value_t = String::from("apdu.dat"))]
    fileout: String
}

use apdu_generator::apdu::Apdu;

fn main() {

    let args: Args = Args::parse();
    
    let mut file = File::open(args.json).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let mut tx: apdu_generator::types::Tx = serde_json::from_str(&data).unwrap();
    tx.calls.reverse();
    
    //let mut file_out = File::create(args.fileout).unwrap();

    let mut apdus: Vec<Apdu> = Vec::new();

    let dpath_apdu = apdu_generator::builder::derivation_path_to_apdu(PATH, args.cla, args.ins.into(), 0);
    apdus.push(dpath_apdu);
    
    
    //println!("{}", serde_json::to_string_pretty(&dpath_apdu).unwrap());
    
    //println!("=> {}", dpath_apdu);
    //writeln!(file_out, "=> {}", dpath_apdu).unwrap();

    let txinfo_apdu = apdu_generator::builder::txinfo_to_apdu(&tx, args.cla, args.ins.into(), 1);
    apdus.push(txinfo_apdu);
    
    println!("{}", serde_json::to_string_pretty(&apdus).unwrap());
    
    /*println!("=> {}",txinfo_apdu);
    writeln!(file_out, "=> {}", txinfo_apdu).unwrap();

    while tx.calls.len() > 0 {
        let call = tx.calls.pop().unwrap();
        let apdu = apdu_generator::builder::call_to_apdu(&call, args.cla, args.ins.into());
        for a in apdu {
            println!("=> {a}");
            writeln!(file_out, "=> {}", a).unwrap();
        }
    }*/
}
