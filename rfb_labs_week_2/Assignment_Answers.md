# Parts 1 and 2: Data Model Review

The remaining written answers (questions 1 to 12), the Part 7 ownership compiler
error, the design notes, and the `cargo run` output are in
[README.md](README.md), which is where the submission standard expects them.


+ OutputType is an enum because a Bitcoin output can only be one script type
  at a time (P2PKH, P2WPKH, P2TR, or OP_RETURN).

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputType {
  P2pkh,
  P2wpkh,
  P2tr,
  OpReturn,
}
```

+ TxOutput stores
  - value (satoshis)
  - recipient address
  - output type

```rust
#[derive(Debug, PartialEq, Eq)]
pub struct TxOutput {
  pub value: u64,
  pub recipient: String,
  pub output_type: OutputType,
}
```


+ OutPoint identifies a previous output being spent using
  - txid
  - vout

```rust
#[derive(Debug, PartialEq, Eq)]
pub struct OutPoint {
  pub txid: String,
  pub vout: u32,
}
```

+ `InputKind` is an enum because Bitcoin has two fundamentally different kinds of inputs:

```rust
Regular {
  previous_output,
  value,
  sequence,
}
```

```rust
Coinbase {
  block_height,
  reward,
}
```

```rust
#[derive(Debug, PartialEq, Eq)]
pub enum InputKind {
  Regular {
    previous_output: OutPoint,
    value: u64,
    sequence: u32,
  },
  Coinbase {
    block_height: u32,
    reward: u64,
  },
}
```

```rust
#[derive(Debug, PartialEq, Eq)]
pub struct Transaction {
  pub version: i32,
  pub inputs: Vec<InputKind>,
  pub outputs: Vec<TxOutput>,
  pub locktime: u32,
}
```


Regular inputs spend previous outputs.

Coinbase inputs create new coins as the mining reward.

Because InputKind is an enum, Rust forces you to match every variant. For example now:

```rust
match input {
  InputKind::Regular { value, .. } => ...
  InputKind::Coinbase { reward, .. } => ...
}
```

If another variant is added later, the compiler reports every place
that must be updated, making the code much safer.

Bitcoin transaction inputs are not inherently enums; rather, the types of inputs or outputs are identified by their script formats (such as P2PKH or P2SH) which are often represented as enums in software libraries for type safety. 

+ **No Native Type Field**: The Bitcoin protocol does not include a specific "type" field in the transaction data structure. Inputs and outputs are defined by their scriptPubKey and scriptSig. 

+ **Software Representation**: Developers use enums (like `SigHashType` or output type identifiers) in code to categorize these scripts because enums provide compile-time type safety, prevent typos, and clearly define a fixed set of valid options (e.g., legacy, SegWit, Taproot). 

+ **RPC Interpretation**: Bitcoin RPC tools generate "type" fields in JSON output by analyzing the script content, using internal enumerations to map raw script bytes to human-readable labels like pubkeyhash or scripthash. 

In summary, the enum is a programming abstraction used to handle and display the diverse script types found in inputs, not a fundamental part of the Bitcoin blockchain's data format.


## How matching forces both variants to be handled

Because `InputKind` is an enum rather than a struct with optional fields, every
`match` over it must cover both arms or the code does not compile. Two places in
this crate rely on that:

```rust
// src/transaction.rs, BitcoinValue for InputKind
match self {
  InputKind::Regular { value, .. } => *value,
  InputKind::Coinbase { reward, .. } => *reward,
}
```

```rust
// src/transaction.rs, validate()
match input {
  InputKind::Regular { previous_output, .. } => {
    regular_count += 1;
    if previous_output.txid.trim().is_empty() {
      return Err(TransactionError::InvalidTxid);
    }
  }
  InputKind::Coinbase { .. } => {
    coinbase_count += 1;
  }
}
```

Dropping either arm produces `error[E0004]: non-exhaustive patterns`. Counting
both kinds separately is also what makes the
`CoinbaseMixedWithRegularInputs` and `MultipleCoinbaseInputs` checks possible in a
single pass.

Each variant carries only its own fields, so a coinbase input cannot accidentally
hold an outpoint and a regular input cannot hold a block height. A single struct
with `Option` fields would allow both of those impossible states at compile time
and push the checking to runtime.
