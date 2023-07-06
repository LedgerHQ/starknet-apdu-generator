use crate::apdu::{ApduHeader, Apdu};
use crate::types::{Call, FieldElement, Tx, Ins};
use ethereum_types::U256;

mod builder_internal;
use builder_internal::fix;

pub enum ApduError {
    InternalError
}

/*pub fn get_version_apdus() -> Result<Vec<Apdu>, ApduError> {
    Ok(vec![Apdu::new(ApduHeader {cla: 0x80, ins: 0x00, p1: 0x00, p2: 0x00})])
}

pub fn get_pubkey_apdus(path: &str) -> Result<Vec<Apdu>, ApduError> {
    let header: ApduHeader = ApduHeader {
        cla: 0x80, 
        ins: 0x01,
        p1: 0x00, 
        p2: 0x00
    };
    let apdu = set_derivation_path(path, header);
    Ok(vec![apdu])
}
*/

pub fn pedersenhash_to_apdu(hash: &str, cla: u8, ins: Ins, sub_ins: u8, show_hash: bool) -> Apdu {

    let header: ApduHeader = ApduHeader {
        cla: cla, 
        ins: ins.into(),
        p1: sub_ins, 
        p2: match show_hash {
            true => 0x01,
            false => 0x00,
        }
    };
    let mut apdu = Apdu::new(header);

    let mut fixed_hash = String::from(hash.trim_start_matches("0x"));
    fix(&mut fixed_hash);
    let data: [u8; 32] = FieldElement(U256::from_str_radix(fixed_hash.as_str(), 16).unwrap())
        .try_into()
        .unwrap();
    apdu.append(&data[..]).unwrap();
    apdu
}


/// Build Derivation path APDU
pub fn derivation_path_to_apdu(path: &str, cla: u8, ins: Ins, sub_ins: u8) -> Apdu {

    let apdu_header = ApduHeader { cla: cla, ins: ins.into(), p1: sub_ins, p2: 0x00 };
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

pub fn txinfo_to_apdu (
    tx: &Tx,
    cla: u8, ins: Ins, sub_ins: u8
) -> Apdu {

    let apdu_header = ApduHeader { cla: cla, ins: ins.into(), p1: sub_ins, p2: 0x00 };
    let mut apdu = Apdu::new(apdu_header);

    let mut fe: FieldElement = FieldElement(U256::from_str_radix(&tx.sender_address, 16).unwrap());    
    let mut data: [u8; 32] = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    fe = FieldElement(U256::from_str_radix(&tx.max_fee, 10).unwrap());
    data = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    fe = FieldElement(U256::from_str_radix(&tx.chain_id, 16).unwrap());
    data = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    fe = FieldElement(U256::from_str_radix(&tx.nonce, 10).unwrap());
    data = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    fe = FieldElement(U256::from_str_radix(&tx.version, 10).unwrap());
    data = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    fe = FieldElement(U256::from(tx.calls.len()));
    data = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    apdu
}

pub fn call_to_apdu(call: &Call, cla: u8, ins: Ins) -> Vec<Apdu> {

    let mut apdu_list: Vec<Apdu> = Vec::new();
    let mut fe: [u8; 32] = [0u8; 32];
    let data: Vec<FieldElement> = call.into();

    let nb_apdu = data.chunks(7).len();

    match nb_apdu {
        1 => {
            let apdu_header = ApduHeader { cla: cla, ins: ins.into(), p1: 0x02, p2: 0x00 };
            let mut apdu = Apdu::new(apdu_header);

            let data = data.chunks(7).next().unwrap();
            for d in data {
                d.0.to_big_endian(&mut fe);
                apdu.append(&fe).unwrap();
            }
            apdu_list.push(apdu);
        }
        2 => {
            
            let mut iter =  data.chunks(7);

            let mut apdu_header = ApduHeader { cla: 0x80, ins: Ins::SignTx.into(), p1: 0x02, p2: 0x01 };
            let mut apdu = Apdu::new(apdu_header);
            let mut data = iter.next().unwrap();
            for d in data {
                d.0.to_big_endian(&mut fe);
                apdu.append(&fe).unwrap();
            }
            apdu_list.push(apdu);

            apdu_header = ApduHeader { cla: 0x80, ins: Ins::SignTx.into(), p1: 0x02, p2: 0x03 };
            apdu = Apdu::new(apdu_header);
            data = iter.next().unwrap();
            for d in data {
                d.0.to_big_endian(&mut fe);
                apdu.append(&fe).unwrap();
            }
            apdu_list.push(apdu);
        }
        3.. => {
            let mut iter =  data.chunks(7);

            let mut apdu_header = ApduHeader { cla: 0x80, ins: Ins::SignTx.into(), p1: 0x02, p2: 0x01 };
            let mut apdu = Apdu::new(apdu_header);
            let mut data = iter.next().unwrap();
            for d in data {
                d.0.to_big_endian(&mut fe);
                apdu.append(&fe).unwrap();
            }
            apdu_list.push(apdu);

            while iter.len() > 1 {
                apdu_header = ApduHeader { cla: 0x80, ins: Ins::SignTx.into(), p1: 0x02, p2: 0x02 };
                apdu = Apdu::new(apdu_header);
                data = iter.next().unwrap();
                for d in data {
                    d.0.to_big_endian(&mut fe);
                    apdu.append(&fe).unwrap();
                }
                apdu_list.push(apdu);
            }

            apdu_header = ApduHeader { cla: 0x80, ins: Ins::SignTx.into(), p1: 0x02, p2: 0x03 };
            apdu = Apdu::new(apdu_header);
            data = iter.next().unwrap();
            for d in data {
                d.0.to_big_endian(&mut fe);
                apdu.append(&fe).unwrap();
            }
            apdu_list.push(apdu);
        }
        _ => ()
    }
    apdu_list
}