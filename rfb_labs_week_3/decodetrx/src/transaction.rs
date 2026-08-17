use serde::{Serialize, Serializer};
use serde_json::value::RawValue;

// === Types

#[derive(Debug, Serialize)]
pub struct Transaction {
    pub transaction_id: Txid,
    pub version: u32,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub lock_time: u32,
}

#[derive(Debug, Serialize)]
pub struct Input {
    pub txid: Txid, // [u8; 32],
    pub output_index: u32,
    #[serde(serialize_with = "as_hex")]
    pub script_sig: Vec<u8>,
    pub sequence: u32,
    // Empty on legacy inputs, so it is omitted entirely rather than printed as
    // an empty array on every non-SegWit transaction.
    #[serde(serialize_with = "as_hex_items", skip_serializing_if = "Vec::is_empty")]
    pub witness: Vec<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct Output {
    #[serde(serialize_with = "as_btc")]
    pub amount: Amount,
    #[serde(serialize_with = "as_hex")]
    pub script_pubkey: Vec<u8>,
}

#[derive(Debug)]
pub struct Amount(u64);

impl Amount {
    // type associated functiion that initiate the instance of the struct i.e Amount
    pub fn from_sat(satoshi: u64) -> Amount {
        Amount(satoshi)
    }
}

#[derive(Debug)]
pub struct Txid([u8; 32]);

// [u8; 32] => array of 32 element each element is 1 byte [u8]; i.e one byte is u8;

impl Txid {
    pub fn from_bytes(bytes: [u8; 32]) -> Txid {
        Txid(bytes)
    }
}

// === Serialization helpers

fn as_btc<S: Serializer, T: BitcoinValue>(t: &T, s: S) -> Result<S::Ok, S::Error> {
    // serialize_f64 would emit the shortest round-trip form, turning 100 sats
    // into 1e-6. Bitcoin amounts are always shown with 8 decimal places, so the
    // value is written as a raw JSON number instead of going through f64.
    let formatted = format!("{:.8}", t.to_btc());
    let raw = RawValue::from_string(formatted).map_err(serde::ser::Error::custom)?;
    raw.serialize(s)
}

fn as_hex<S: Serializer>(bytes: &[u8], s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&hex::encode(bytes))
}

fn as_hex_items<S: Serializer>(items: &[Vec<u8>], s: S) -> Result<S::Ok, S::Error> {
    s.collect_seq(items.iter().map(hex::encode))
}

impl Serialize for Txid {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        // Txids are stored internally in little-endian but always displayed
        // big-endian, so the byte order has to be flipped for output.
        let mut display = self.0;
        display.reverse();
        s.serialize_str(&hex::encode(display))
    }
}

trait BitcoinValue {
    fn to_btc(&self) -> f64;
}

impl BitcoinValue for Amount {
    fn to_btc(&self) -> f64 {
        self.0 as f64 / 100_000_000.0
    }
}
