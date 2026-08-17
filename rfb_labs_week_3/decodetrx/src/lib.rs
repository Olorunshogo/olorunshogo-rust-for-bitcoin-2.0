use sha2::{Digest, Sha256}; // https://docs.rs/sha2/latest/sha2/
use std::io::{Error, ErrorKind, Read};
use transaction::{Amount, Input, Output, Transaction, Txid};
mod transaction;

// Bitcoin uses little-endian encoding for most of its numeric fields, meaning the least significant byte comes first.

// === Readers

fn read_u32(bytes_slice: &mut &[u8]) -> Result<u32, Error> {
    let mut buffer = [0u8; 4];
    bytes_slice.read_exact(&mut buffer)?;
    Ok(u32::from_le_bytes(buffer))
}

fn read_u64(transaction_bytes: &mut &[u8]) -> Result<u64, Error> {
    let mut buffer = [0u8; 8];
    transaction_bytes.read_exact(&mut buffer)?;
    Ok(u64::from_le_bytes(buffer))
}

fn read_amount(transaction_bytes: &mut &[u8]) -> Result<Amount, Error> {
    Ok(Amount::from_sat(read_u64(transaction_bytes)?))
}

// CompactSize integers encode the number of inputs, outputs, script lengths and
// witness element counts, so this is the workhorse of the whole parser.
fn read_compact_size(transaction_bytes: &mut &[u8]) -> Result<u64, Error> {
    let mut marker = [0u8; 1];
    transaction_bytes.read_exact(&mut marker)?;

    match marker[0] {
        0xfd => {
            let mut buffer = [0u8; 2];
            transaction_bytes.read_exact(&mut buffer)?;
            Ok(u16::from_le_bytes(buffer) as u64)
        }
        0xfe => Ok(read_u32(transaction_bytes)? as u64),
        0xff => read_u64(transaction_bytes),
        n => Ok(n as u64),
    }
}

fn read_txid(transaction_bytes: &mut &[u8]) -> Result<Txid, Error> {
    let mut buffer = [0u8; 32];
    transaction_bytes.read_exact(&mut buffer)?;
    Ok(Txid::from_bytes(buffer))
}

// A CompactSize length is attacker-controlled and can claim up to u64::MAX, so
// it is checked against what is actually left before anything is allocated.
fn read_bytes(transaction_bytes: &mut &[u8], size: u64) -> Result<Vec<u8>, Error> {
    if size > transaction_bytes.len() as u64 {
        return Err(Error::new(
            ErrorKind::UnexpectedEof,
            format!(
                "declared length {size} exceeds the {} remaining byte(s)",
                transaction_bytes.len()
            ),
        ));
    }

    let mut buffer = vec![0u8; size as usize];
    transaction_bytes.read_exact(&mut buffer)?;
    Ok(buffer)
}

// Reads a CompactSize length prefix followed by that many script bytes.
fn read_script(transaction_bytes: &mut &[u8]) -> Result<Vec<u8>, Error> {
    let script_size = read_compact_size(transaction_bytes)?;
    read_bytes(transaction_bytes, script_size)
}

fn hash_row_transaction(row_transaction_bytes: &[u8]) -> Result<Txid, Error> {
    let first_hash = Sha256::digest(row_transaction_bytes);
    let second_hash = Sha256::digest(first_hash);

    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&second_hash);
    Ok(Txid::from_bytes(bytes))
}

// === Decoder

pub fn decode_transaction(transaction_hex: String) -> Result<String, Box<dyn std::error::Error>> {
    let transaction_bytes = hex::decode(transaction_hex.trim())?;
    let total_length = transaction_bytes.len();
    let mut bytes_slice = transaction_bytes.as_slice();

    let version = read_u32(&mut bytes_slice)?;

    /* A zero where the input count belongs is the SegWit marker; the flag byte follows it. 
        Neither carries information the decoder needs, so both are consumed and the real input count is read after them.
     */
    
    let is_segwit = bytes_slice.first() == Some(&0x00);
    if is_segwit {
        if bytes_slice.len() < 2 {
            return Err(Box::new(Error::new(
                ErrorKind::UnexpectedEof,
                "truncated segwit marker",
            )));
        }
        bytes_slice = &bytes_slice[2..];
    }

    let input_count = read_compact_size(&mut bytes_slice)?;
    let mut inputs = Vec::new();
    for _ in 0..input_count {
        let txid = read_txid(&mut bytes_slice)?;
        let output_index = read_u32(&mut bytes_slice)?;
        let script_sig = read_script(&mut bytes_slice)?;
        let sequence = read_u32(&mut bytes_slice)?;

        inputs.push(Input {
            txid,
            output_index,
            script_sig,
            sequence,
            // Witness data trails the outputs, so it is filled in below.
            witness: Vec::new(),
        });
    }

    let output_count = read_compact_size(&mut bytes_slice)?;
    let mut outputs = Vec::new();
    for _ in 0..output_count {
        let amount = read_amount(&mut bytes_slice)?;
        let script_pubkey = read_script(&mut bytes_slice)?;

        outputs.push(Output {
            amount,
            script_pubkey,
        });
    }

    // Offset of the byte just past the outputs, needed to cut the witness block
    // out when rebuilding the legacy serialization.
    let end_of_outputs = total_length - bytes_slice.len();

    if is_segwit {
        // Each input carries its own witness stack, in input order.
        for input in inputs.iter_mut() {
            let item_count = read_compact_size(&mut bytes_slice)?;
            for _ in 0..item_count {
                let item_size = read_compact_size(&mut bytes_slice)?;
                input.witness.push(read_bytes(&mut bytes_slice, item_size)?);
            }
        }
    }

    let lock_time = read_u32(&mut bytes_slice)?;

    // Locktime is the last field, so anything left over means the hex did not
    // describe exactly one transaction.
    if !bytes_slice.is_empty() {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidData,
            format!("{} trailing byte(s) after locktime", bytes_slice.len()),
        )));
    }

    // The txid is always the double-SHA256 of the legacy serialization, so the
    // marker, flag and witness block must be stripped before hashing.
    let transaction_id = if is_segwit {
        let mut legacy = Vec::with_capacity(total_length);
        legacy.extend_from_slice(&transaction_bytes[0..4]);
        legacy.extend_from_slice(&transaction_bytes[6..end_of_outputs]);
        legacy.extend_from_slice(&transaction_bytes[total_length - 4..]);
        hash_row_transaction(&legacy)?
    } else {
        hash_row_transaction(&transaction_bytes)?
    };

    let transaction = Transaction {
        transaction_id,
        version,
        inputs,
        outputs,
        lock_time,
    };

    Ok(serde_json::to_string_pretty(&transaction)?)
}

// === Tests

#[cfg(test)]
mod tests {
    use super::*;

    // A native P2WPKH testnet spend: 1 input, 2 outputs, witness on the single input.
    const SEGWIT_TX: &str = "0200000000010196277c04c986c1ad78c909287fd12dba2924324699a0232e0533f46a6a3916bb0100000000ffffffff026400000000000000160014274ae586ad2035efb4c25049c155f98310d7e106ca16440000000000160014599bcef6387256c6b019030c421b4a4d382fe2600247304402204d94a1e4047ca38a450177ccb6f88585ca147f1939df343d8ac5d962c5f35bb302206f7fa42c21c47ebccdc460393d35c5dfd3b6f0a26cf10fac23d3e6fab71835c20121020cb972a66e3fb1cdcc9efcad060b4457ebec534942700d4af1c0d82a33aa13f100000000";

    // Block 170, the Satoshi to Hal Finney transaction.
    const LEGACY_TX: &str = "0100000001c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd3704000000004847304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901ffffffff0200ca9a3b00000000434104ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84cac00286bee0000000043410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac00000000";

    #[test]
    fn compact_size_boundaries() {
        assert_eq!(read_compact_size(&mut [0xfcu8].as_slice()).unwrap(), 252);
        assert_eq!(
            read_compact_size(&mut [0xfd, 0x00, 0x01].as_slice()).unwrap(),
            256
        );
        assert_eq!(
            read_compact_size(&mut [0xfe, 0x00, 0x00, 0x01, 0x00].as_slice()).unwrap(),
            65536
        );
        assert_eq!(
            read_compact_size(&mut [0xff, 0, 0, 0, 0, 0x01, 0, 0, 0].as_slice()).unwrap(),
            4294967296
        );
    }

    #[test]
    fn compact_size_rejects_truncated_input() {
        assert!(read_compact_size(&mut [0xfd, 0x00].as_slice()).is_err());
    }

    #[test]
    fn decodes_segwit_transaction() {
        let json = decode_transaction(SEGWIT_TX.to_string()).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(
            value["transaction_id"],
            "be9ea29072566edbc6827e3d9caf1d8c0b57cb0d5e74b95c721c46b3124cbe0b"
        );
        assert_eq!(value["version"], 2);
        assert_eq!(value["lock_time"], 0);
        assert_eq!(value["inputs"].as_array().unwrap().len(), 1);
        assert_eq!(value["outputs"].as_array().unwrap().len(), 2);
        // scriptSig is empty on a native SegWit spend.
        assert_eq!(value["inputs"][0]["script_sig"], "");
        assert_eq!(value["inputs"][0]["output_index"], 1);
        assert_eq!(value["outputs"][0]["amount"], 0.000001);
    }

    #[test]
    fn decodes_legacy_transaction() {
        let json = decode_transaction(LEGACY_TX.to_string()).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(
            value["transaction_id"],
            "f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16"
        );
        assert_eq!(value["version"], 1);
        assert_eq!(value["inputs"].as_array().unwrap().len(), 1);
        assert_eq!(value["outputs"].as_array().unwrap().len(), 2);
        assert_eq!(value["outputs"][0]["amount"], 10.0);
        assert_eq!(value["outputs"][1]["amount"], 40.0);
    }

    #[test]
    fn amounts_use_eight_decimal_places() {
        let json = decode_transaction(SEGWIT_TX.to_string()).unwrap();
        assert!(json.contains("\"amount\": 0.00000100"));
        assert!(json.contains("\"amount\": 0.04462282"));

        let json = decode_transaction(LEGACY_TX.to_string()).unwrap();
        assert!(json.contains("\"amount\": 10.00000000"));
    }

    #[test]
    fn decodes_witness_stack() {
        let json = decode_transaction(SEGWIT_TX.to_string()).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let witness = value["inputs"][0]["witness"].as_array().unwrap();

        // A P2WPKH spend witness is exactly [signature, pubkey].
        assert_eq!(witness.len(), 2);
        assert_eq!(
            witness[0],
            "304402204d94a1e4047ca38a450177ccb6f88585ca147f1939df343d8ac5d962c5f35bb302206f7fa42c21c47ebccdc460393d35c5dfd3b6f0a26cf10fac23d3e6fab71835c201"
        );
        assert_eq!(
            witness[1],
            "020cb972a66e3fb1cdcc9efcad060b4457ebec534942700d4af1c0d82a33aa13f1"
        );
    }

    #[test]
    fn legacy_input_omits_witness_field() {
        let json = decode_transaction(LEGACY_TX.to_string()).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(value["inputs"][0].get("witness").is_none());
    }

    #[test]
    fn rejects_oversized_declared_length() {
        // Version, 1 input, a txid, vout, then a scriptSig claiming 0xffffffffffffffff bytes.
        let crafted = format!(
            "01000000 01 {} 00000000 ff ffffffffffffffff",
            "11".repeat(32)
        )
        .replace(' ', "");
        let error = decode_transaction(crafted).unwrap_err().to_string();
        assert!(error.contains("exceeds"), "unexpected error: {error}");
    }

    #[test]
    fn rejects_trailing_bytes() {
        let padded = format!("{SEGWIT_TX}00");
        assert!(decode_transaction(padded).is_err());

        let padded = format!("{LEGACY_TX}deadbeef");
        assert!(decode_transaction(padded).is_err());
    }

    #[test]
    fn rejects_invalid_hex() {
        assert!(decode_transaction("zzzz".to_string()).is_err());
    }

    #[test]
    fn rejects_truncated_transaction() {
        assert!(decode_transaction("0200".to_string()).is_err());
    }
}
