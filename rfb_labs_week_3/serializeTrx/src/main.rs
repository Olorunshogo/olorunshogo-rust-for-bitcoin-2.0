use std::error::Error;

#[derive(Debug)]
struct TxInput {
    prev_txid: Vec<u8>,
    vout: u32,
    script_sig: Vec<u8>,
    sequence: u32,
    witness: Vec<Vec<u8>>,
}


#[derive(Debug)]
struct TxOutput {
    value: u64,
    script_pubkey: Vec<u8>,
}


#[derive(Debug)]
struct Transaction {
    version: i32,
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
    locktime: u32,
    segwit: bool,
}


fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    if hex.len() % 2 != 0 {
        return Err("Hex string must have even length".into());
    }

    // create vector with enough bytes capacity
    let mut bytes = Vec::with_capacity(hex.len() / 2);

    for i in (0..hex.len()).step_by(2) {
        // Give me the next two hexadecimal characters.
        // Convert the two hex characters into a byte
        let byte = u8::from_str_radix(&hex[i..i + 2], 16)?;
        // from_str_radix - Parse a string as a number using a particular base i.e 16
        bytes.push(byte);
    }

    Ok(bytes)
}


fn main() -> Result<(), Box<dyn Error>> { 

    let input = TxInput {
        prev_txid: hex_to_bytes(
            "8fb0d07bb3766421bff2d908b70e5de818e4d85a436ea3606310c1052b0dc821"
        )?,
        vout: 1,
        script_sig: vec![],
        sequence: 0xffffffff,
        witness: vec![
            hex_to_bytes("3045022100f8704a3e7d55d4b5ee448cc6365caeffa42c2b00f74a37726d4fa3c11982e3e502203591c4a4bde9200281755ae5a8759116ce6e0cc7f5d30cf0eeb5b2b74f74bab301")?,
            hex_to_bytes("029cbb1e568de08f469a8751aa2000331f130ca92ad49012d9cececaf6f8eb2358")?   
        ]
    };

    let output_0 = TxOutput {
        value: 69886,
        script_pubkey: hex_to_bytes("0014a632c1fff47af29f8c81dc4c6e91eb49a116c12b")?,
    };

    let output_1 = TxOutput {
        value: 29442,
        script_pubkey: hex_to_bytes("00149831122b93d21715c70db626ccc844d3c21f9687")?,
    };
    
    let trx = Transaction {
        version : 2,
        inputs: vec![input],
        outputs: vec![output_0, output_1],
        locktime: 0,
        segwit: true
    };

       // Serialize
    let serialized = serialize_transaction(&trx);

    println!("Serialized transaction:");
    println!("{:?}", &serialized);
    println!("Serialized Hex transaction:");
    println!("{}", bytes_to_hex(&serialized));

    println!("\nTransaction size: {} bytes", serialized.len());

    Ok(())

}   

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}



// ┌──────────────────────────────┐
// │ Version          4 bytes     │
// ├──────────────────────────────┤
// │ Marker           1 byte      │
// │ Flag             1 byte      │
// ├──────────────────────────────┤
// │ Input count      VarInt      │
// │ Inputs           Variable    │
// ├──────────────────────────────┤
// │ Output count     VarInt      │
// │ Outputs          Variable    │
// ├──────────────────────────────┤
// │ Witness          Variable    │
// ├──────────────────────────────┤
// │ Locktime         4 bytes  ←  │
// └──────────────────────────────┘


fn serialize_transaction(trx: &Transaction) -> Vec<u8> {

     let mut result = Vec::new();

        // add version number
          // to_le_bytes: converts the integer into its little-endian byte representation.
    //  extend_from_slice: Take these bytes and append them to result.
        result.extend_from_slice(&trx.version.to_le_bytes());

     if trx.segwit {
        result.push(0x00); // marker
        result.push(0x01); // flag
     };

     // INPUTT COUNT
      // script_sig: vec![] is empty because this particular transaction is a SegWit P2WPKH transaction.
        // scriptSig belongs to the traditional input structure.
        // witness contains the signature and public key for a native SegWit input.
     result.extend_from_slice(&encode_varint(trx.inputs.len()));

     // input data 
        for input in &trx.inputs {
        // Previous transaction ID
        result.extend_from_slice(&input.prev_txid);

        // Previous output index
        result.extend_from_slice(&input.vout.to_le_bytes());

        // ScriptSig length
        result.extend_from_slice(&encode_varint(input.script_sig.len()));

        // ScriptSig
        result.extend_from_slice(&input.script_sig);

        // Sequence
        result.extend_from_slice(&input.sequence.to_le_bytes());
    }
    // OUTPUT COUNT
    result.extend_from_slice(&encode_varint(trx.outputs.len()));

    // OUTPUT DATA 
        for output in &trx.outputs {
        // Value in satoshis
        result.extend_from_slice(&output.value.to_le_bytes());

        // ScriptPubKey length
        result.extend_from_slice(&encode_varint(output.script_pubkey.len()));

        // ScriptPubKey
        result.extend_from_slice(&output.script_pubkey);
    }

    // witness data
       if trx.segwit {
        for input in &trx.inputs {
            // Number of witness items
            result.extend_from_slice(&encode_varint(input.witness.len()));

            for item in &input.witness {
                // Witness item length
                result.extend_from_slice(&encode_varint(item.len()));

                // Witness item
                result.extend_from_slice(item);
            }
        }
    }

    // add locktime 
    result.extend_from_slice(&trx.locktime.to_le_bytes());

    result 

}

// Bitcoin uses VarInts (encode_varint) when it needs to store things like:

// number of inputs
// number of outputs
// script length
// number of witness items
// witness item length


fn encode_varint(value: usize) -> Vec<u8> {
    match value {
        0..=0xfc => vec![value as u8],

        0xfd..=0xffff => {
            let mut result = vec![0xfd];
            result.extend_from_slice(&(value as u16).to_le_bytes());
            result
        }

        0x10000..=0xffff_ffff => {
            let mut result = vec![0xfe];
            result.extend_from_slice(&(value as u32).to_le_bytes());
            result
        }

        _ => {
            let mut result = vec![0xff];
            result.extend_from_slice(&(value as u64).to_le_bytes());
            result
        }
    }
}

// Bitcoin CompactSize follows this structure:
// Value range          Encoding

// 0 - 252              1 byte

// 253 - 65,535         FD + 2 bytes

// 65,536 - 4,294,967,295
//                      FE + 4 bytes

// larger values        FF + 8 bytes


// A simpler way to visualize CompactSize
//               ┌── small value?
//               │
//               ↓
//            0 - 252 (0xfc)
//               │
//               └── store directly
//                     ↓
//                    [XX]


//            253 - 65535
//               │
//               └── FD + 2 bytes
//                     ↓
//                  [FD][XX XX]


//            65536 - 4294967295
//               │
//               └── FE + 4 bytes
//                     ↓
//               [FE][XX XX XX XX]


//            larger
//               │
//               └── FF + 8 bytes
//                     ↓
//           [FF][XX XX XX XX XX XX XX XX]