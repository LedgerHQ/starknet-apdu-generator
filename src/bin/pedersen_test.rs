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
    #[arg(short, long, default_value_t = 0x04)]
    ins: u8
}

use starknet_apdu_generator::{
    apdu::Apdu,
    types::{Data, FieldElement},
    builder
};

fn main() {

    let args: Args = Args::parse();
    
    let mut file = File::open(args.json).unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();

    let mut data: Data = serde_json::from_str(&file_content).unwrap();

    println!("{}", data.felts.len());
    let mut felts: Vec<FieldElement> = vec![];
    for f in data.felts {
        println!("{}", f);
        let f = FieldElement::try_from(f.as_str()).unwrap();
        felts.push(f);
    }
    
    let mut apdus: Vec<Apdu> = Vec::new();

    apdus.push(builder::data_to_apdu(felts, args.cla, args.ins, 0x00, 0x00));

    let mut json_out = File::create("apdu.json").unwrap();
    let mut raw_out = File::create("apdu.dat").unwrap();
    for a in apdus.iter() {
        println!("=> {}", a);
        writeln!(raw_out, "=> {}", a).unwrap();
    }
    writeln!(json_out, "{}", serde_json::to_string_pretty(&apdus).unwrap()).unwrap();
    
}
