use crate::transaction::Transaction;

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

pub fn serialize_transaction(trx: &Transaction) -> Vec<u8> {
    let mut result = Vec::new();

    // add version number
    // to_le_bytes: converts the integer into its little-endian byte representation.
    // extend_from_slice: Take these bytes and append them to result.
    result.extend_from_slice(&trx.version.to_le_bytes());

    if trx.segwit {
        result.push(0x00); // marker
        result.push(0x01); // flag
    };

    // == Input COUNT
    // script_sig is empty for a SegWit P2WPKH transaction.
    // scriptSig belongs to the traditional input structure.
    // witness contains the signature and public key for a native SegWit input.
    result.extend_from_slice(&encode_varint(trx.inputs.len()));

    // == Input data
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

    // == Output COUNT
    result.extend_from_slice(&encode_varint(trx.outputs.len()));

    // == Output DATA
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

pub fn encode_varint(value: usize) -> Vec<u8> {
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

// === Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hex::{bytes_to_hex, hex_to_bytes};
    use crate::transaction::{TxInput, TxOutput};

    #[test]
    fn varint_boundaries() {
        assert_eq!(encode_varint(0), vec![0x00]);
        assert_eq!(encode_varint(252), vec![0xfc]);
        assert_eq!(encode_varint(253), vec![0xfd, 0xfd, 0x00]);
        assert_eq!(encode_varint(65535), vec![0xfd, 0xff, 0xff]);
        assert_eq!(encode_varint(65536), vec![0xfe, 0x00, 0x00, 0x01, 0x00]);
        assert_eq!(
            encode_varint(4294967296),
            vec![0xff, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00]
        );
    }

    /* Pins the serialization logic to what the pre-refactor program produced.
        The txid bytes are passed in exactly as the old hardcoded version held
        them, so this checks the writer alone and is unaffected by the byte-order
        fix that now happens during argument parsing.
    */
    #[test]
    fn matches_pre_refactor_output() {
        let input = TxInput {
            prev_txid: hex_to_bytes(
                "8fb0d07bb3766421bff2d908b70e5de818e4d85a436ea3606310c1052b0dc821",
            )
            .unwrap(),
            vout: 1,
            script_sig: vec![],
            sequence: 0xffffffff,
            witness: vec![
                hex_to_bytes("3045022100f8704a3e7d55d4b5ee448cc6365caeffa42c2b00f74a37726d4fa3c11982e3e502203591c4a4bde9200281755ae5a8759116ce6e0cc7f5d30cf0eeb5b2b74f74bab301").unwrap(),
                hex_to_bytes("029cbb1e568de08f469a8751aa2000331f130ca92ad49012d9cececaf6f8eb2358").unwrap(),
            ],
        };

        let trx = Transaction {
            version: 2,
            inputs: vec![input],
            outputs: vec![
                TxOutput {
                    value: 69886,
                    script_pubkey: hex_to_bytes("0014a632c1fff47af29f8c81dc4c6e91eb49a116c12b")
                        .unwrap(),
                },
                TxOutput {
                    value: 29442,
                    script_pubkey: hex_to_bytes("00149831122b93d21715c70db626ccc844d3c21f9687")
                        .unwrap(),
                },
            ],
            locktime: 0,
            segwit: true,
        };

        let serialized = serialize_transaction(&trx);

        assert_eq!(
            bytes_to_hex(&serialized),
            "020000000001018fb0d07bb3766421bff2d908b70e5de818e4d85a436ea3606310c1052b0dc8210100000000ffffffff02fe10010000000000160014a632c1fff47af29f8c81dc4c6e91eb49a116c12b02730000000000001600149831122b93d21715c70db626ccc844d3c21f968702483045022100f8704a3e7d55d4b5ee448cc6365caeffa42c2b00f74a37726d4fa3c11982e3e502203591c4a4bde9200281755ae5a8759116ce6e0cc7f5d30cf0eeb5b2b74f74bab30121029cbb1e568de08f469a8751aa2000331f130ca92ad49012d9cececaf6f8eb235800000000"
        );
        assert_eq!(serialized.len(), 223);
    }

    #[test]
    fn legacy_transaction_omits_marker_and_witness() {
        let trx = Transaction {
            version: 1,
            inputs: vec![TxInput {
                prev_txid: vec![0x11; 32],
                vout: 0,
                script_sig: vec![0xaa],
                sequence: 0xffffffff,
                witness: vec![],
            }],
            outputs: vec![TxOutput {
                value: 1,
                script_pubkey: vec![0xbb],
            }],
            locktime: 0,
            segwit: false,
        };

        let hex = bytes_to_hex(&serialize_transaction(&trx));
        assert!(!hex.starts_with("010000000001"));
        assert_eq!(
            hex,
            "01000000011111111111111111111111111111111111111111111111111111111111111111000000\
             0001aaffffffff010100000000000000
             01bb00000000"
                .replace(['\n', ' '], "")
        );
    }
}

    