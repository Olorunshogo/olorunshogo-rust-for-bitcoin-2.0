use crate::hex::hex_to_bytes;
use crate::transaction::{Transaction, TxInput, TxOutput};
use clap::Parser;

// 21,000,000 BTC expressed in satoshis, the total supply cap.
const MAX_SATS: u64 = 21_000_000 * 100_000_000;

const TXID_HEX_LEN: usize = 64;

#[derive(Parser)]
#[command(name = "serializeTrx")]
#[command(version = "1.0")]
#[command(about = "Bitcoin transaction serializer", long_about = None)]
pub struct Cli {
    #[arg(long, default_value_t = 2, help = "Transaction version")]
    pub tx_version: i32,

    #[arg(
        long,
        help = "Serialize as a SegWit transaction (marker, flag, witness)"
    )]
    pub segwit: bool,

    #[arg(
        long = "input",
        required = true,
        value_parser = parse_input,
        help = "txid:vout[:scriptSig_hex[:sequence]], repeatable. txid in display order"
    )]
    pub inputs: Vec<TxInput>,

    #[arg(
        long = "output",
        required = true,
        value_parser = parse_output,
        help = "satoshis:scriptPubKey_hex, repeatable"
    )]
    pub outputs: Vec<TxOutput>,

    #[arg(
        long = "witness",
        value_parser = parse_witness,
        help = "Comma-separated hex items for one input, repeated in input order"
    )]
    pub witnesses: Vec<Vec<Vec<u8>>>,

    #[arg(long, default_value_t = 0, help = "Transaction locktime")]
    pub locktime: u32,
}

impl Cli {
    pub fn into_transaction(self) -> Result<Transaction, String> {
        if self.segwit {
            if self.witnesses.len() != self.inputs.len() {
                return Err(format!(
                    "--segwit needs one --witness per input: got {} witness stack(s) for {} input(s)",
                    self.witnesses.len(),
                    self.inputs.len()
                ));
            }
        } else if !self.witnesses.is_empty() {
            return Err("--witness requires --segwit".to_string());
        }

        let mut inputs = self.inputs;
        for (input, witness) in inputs.iter_mut().zip(self.witnesses) {
            input.witness = witness;
        }

        Ok(Transaction {
            version: self.tx_version,
            inputs,
            outputs: self.outputs,
            locktime: self.locktime,
            segwit: self.segwit,
        })
    }
}

// === Value parsers

fn parse_input(value: &str) -> Result<TxInput, String> {
    let fields: Vec<&str> = value.split(':').collect();
    if fields.len() < 2 || fields.len() > 4 {
        return Err(format!(
            "expected txid:vout[:scriptSig_hex[:sequence]], got {} field(s)",
            fields.len()
        ));
    }

    if fields[0].len() != TXID_HEX_LEN {
        return Err(format!(
            "txid must be {TXID_HEX_LEN} hex characters, got {}",
            fields[0].len()
        ));
    }

    // Explorers show txids in display order; the wire format stores them
    // reversed, so the bytes are flipped on the way in.
    let mut prev_txid = hex_to_bytes(fields[0]).map_err(|e| format!("txid: {e}"))?;
    prev_txid.reverse();

    let vout: u32 = fields[1]
        .parse()
        .map_err(|_| format!("vout '{}' is not a valid u32", fields[1]))?;

    let script_sig = match fields.get(2) {
        Some(hex) => hex_to_bytes(hex).map_err(|e| format!("scriptSig: {e}"))?,
        None => Vec::new(),
    };

    let sequence = match fields.get(3) {
        Some(text) => parse_u32(text).map_err(|e| format!("sequence: {e}"))?,
        None => 0xffffffff,
    };

    Ok(TxInput {
        prev_txid,
        vout,
        script_sig,
        sequence,
        // Filled in from ``--witness`` once every input is known.
        witness: Vec::new(),
    })
}

fn parse_output(value: &str) -> Result<TxOutput, String> {
    let fields: Vec<&str> = value.split(':').collect();
    if fields.len() != 2 {
        return Err(format!(
            "expected satoshis:scriptPubKey_hex, got {} field(s)",
            fields.len()
        ));
    }

    let amount: u64 = fields[0]
        .parse()
        .map_err(|_| format!("amount '{}' is not a valid number of satoshis", fields[0]))?;

    if amount > MAX_SATS {
        return Err(format!(
            "amount {amount} sats exceeds the 21,000,000 BTC supply cap"
        ));
    }

    let script_pubkey = hex_to_bytes(fields[1]).map_err(|e| format!("scriptPubKey: {e}"))?;

    Ok(TxOutput {
        value: amount,
        script_pubkey,
    })
}

fn parse_witness(value: &str) -> Result<Vec<Vec<u8>>, String> {
    if value.is_empty() {
        return Ok(Vec::new());
    }

    value
        .split(',')
        .map(|item| hex_to_bytes(item).map_err(|e| format!("witness item: {e}")))
        .collect()
}

// Sequence is conventionally written in hex, so both forms are accepted.
fn parse_u32(text: &str) -> Result<u32, String> {
    let parsed = match text.strip_prefix("0x") {
        Some(rest) => u32::from_str_radix(rest, 16),
        None => text.parse(),
    };

    parsed.map_err(|_| format!("'{text}' is not a valid u32"))
}

// === Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hex::bytes_to_hex;

    #[test]
    fn reverses_txid_into_wire_order() {
        let input =
            parse_input("8fb0d07bb3766421bff2d908b70e5de818e4d85a436ea3606310c1052b0dc821:1")
                .unwrap();

        assert_eq!(
            bytes_to_hex(&input.prev_txid),
            "21c80d2b05c1106360a36e435ad8e418e85d0eb708d9f2bf216476b37bd0b08f"
        );
        assert_eq!(input.vout, 1);
        assert_eq!(input.script_sig, Vec::<u8>::new());
        assert_eq!(input.sequence, 0xffffffff);
    }

    #[test]
    fn accepts_optional_input_fields() {
        let txid = "11".repeat(32);
        let input = parse_input(&format!("{txid}:0:aabb:0xfffffffd")).unwrap();
        assert_eq!(input.script_sig, vec![0xaa, 0xbb]);
        assert_eq!(input.sequence, 0xfffffffd);

        let input = parse_input(&format!("{txid}:0:aabb:4294967293")).unwrap();
        assert_eq!(input.sequence, 0xfffffffd);
    }

    #[test]
    fn rejects_bad_txid() {
        assert!(parse_input("abc:0").is_err());
        assert!(parse_input(&format!("{}:0", "1".repeat(63))).is_err());
        assert!(parse_input(&format!("{}:0", "z".repeat(64))).is_err());
    }

    #[test]
    fn rejects_bad_output() {
        assert!(parse_output("1000").is_err());
        assert!(parse_output("notanumber:0014aa").is_err());
        assert!(parse_output("1000:abc").is_err());
        assert!(parse_output("2100000000000001:00").is_err());
    }

    #[test]
    fn parses_witness_items() {
        assert_eq!(parse_witness("").unwrap(), Vec::<Vec<u8>>::new());
        assert_eq!(
            parse_witness("aa,bbcc").unwrap(),
            vec![vec![0xaa], vec![0xbb, 0xcc]]
        );
        assert!(parse_witness("aa,zz").is_err());
    }

    fn cli_of(inputs: Vec<TxInput>, witnesses: Vec<Vec<Vec<u8>>>, segwit: bool) -> Cli {
        Cli {
            tx_version: 2,
            segwit,
            inputs,
            outputs: vec![TxOutput {
                value: 1,
                script_pubkey: vec![0x00],
            }],
            witnesses,
            locktime: 0,
        }
    }

    #[test]
    fn rejects_witness_count_mismatch() {
        let input = parse_input(&format!("{}:0", "11".repeat(32))).unwrap();
        let cli = cli_of(vec![input], vec![], true);
        assert!(
            cli.into_transaction()
                .unwrap_err()
                .contains("one --witness per input")
        );
    }

    #[test]
    fn rejects_witness_without_segwit() {
        let input = parse_input(&format!("{}:0", "11".repeat(32))).unwrap();
        let cli = cli_of(vec![input], vec![vec![vec![0xaa]]], false);
        assert!(
            cli.into_transaction()
                .unwrap_err()
                .contains("requires --segwit")
        );
    }

    #[test]
    fn attaches_witness_stacks_in_input_order() {
        let first = parse_input(&format!("{}:0", "11".repeat(32))).unwrap();
        let second = parse_input(&format!("{}:1", "22".repeat(32))).unwrap();
        let cli = cli_of(
            vec![first, second],
            vec![vec![vec![0xaa]], vec![vec![0xbb], vec![0xcc]]],
            true,
        );

        let trx = cli.into_transaction().unwrap();
        assert_eq!(trx.inputs[0].witness, vec![vec![0xaa]]);
        assert_eq!(trx.inputs[1].witness, vec![vec![0xbb], vec![0xcc]]);
    }
}
