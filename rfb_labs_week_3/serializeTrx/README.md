# serializeTrx

Builds and serializes a Bitcoin transaction from command-line arguments, then prints the raw bytes, the serialized hex, and the transaction size.

Nothing about the transaction is hardcoded. Version, SegWit status, inputs, outputs, witness data, and locktime all come from flags, so different transactions can be produced without touching the source.

## Running

```
cargo run -- [OPTIONS] --input <INPUT> --output <OUTPUT>
```

## Options

| Flag | Format | Default | Notes |
|---|---|---|---|
| `--tx-version` | integer | `2` | Named `--tx-version` because clap reserves `--version` |
| `--segwit` | flag | off | Adds the `00 01` marker and flag, and the witness block |
| `--input` | `txid:vout[:scriptSig_hex[:sequence]]` | required | Repeatable, order preserved |
| `--output` | `satoshis:scriptPubKey_hex` | required | Repeatable, order preserved |
| `--witness` | `item_hex,item_hex,...` | none | One per input, in input order. Use `''` for an empty stack |
| `--locktime` | integer | `0` | |

Fields are colon separated because hex strings and integers never contain a colon, so no shell quoting is needed.

`scriptSig` defaults to empty and `sequence` defaults to `0xffffffff`, so a minimal input is just `txid:vout`. Sequence accepts either `0xfffffffd` or `4294967293`.

Amounts are in satoshis rather than BTC so that no floating point value ever reaches the serialized bytes.

### txid byte order

`--input` takes txids in **display order**, the way block explorers such as mempool.space show them. Bitcoin stores the previous txid on the wire in reverse, so the program flips the bytes for you.

This is a behavior change. The earlier hardcoded version passed the explorer-order txid straight through to the serializer, which produced the previous txid backwards. Output for the same transaction therefore differs from the pre-refactor program, and the current output is the correct one. You can confirm the direction by feeding the hex to the sibling `decodetrx` crate, which prints input txids back in display order.

## Validation

Bad input is rejected before any bytes are written, and the error names the flag at fault.

- txid must be exactly 64 hexadecimal characters
- every hex value must have even length and contain only hexadecimal digits
- amounts must be a valid `u64` and within the 21,000,000 BTC supply cap
- `vout`, `sequence`, and `locktime` must fit in a `u32`
- with `--segwit`, the number of `--witness` flags must equal the number of `--input` flags
- without `--segwit`, `--witness` is rejected

Example of a rejection:

```
$ cargo run -- --input <txid>:0:483045022100aa02205beef012 --output 1000:00
error: invalid value '...' for '--input <INPUTS>': scriptSig: Hex string must have even length
```

## Examples

### 1. Legacy transaction, one input, one output

```
cargo run -- \
  --tx-version 1 \
  --input c99765e57e10410624fa9c6c2da26d908d05f6259c352efaed2ce5e0a597c904:0:47304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901 \
  --output 1000000000:4104ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84cac
```

```
010000000104c997a5e0e52cedfa2e359c25f6058d906da22d6c9cfa240641107ee56597c90000
00004847304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd
410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901ffffff
ff0100ca9a3b00000000434104ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d
71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84cac
00000000

Transaction size: 199 bytes
```

No `--segwit`, so there is no marker, flag, or witness block.

### 2. SegWit transaction, one input, two outputs

The transaction that used to be hardcoded in `main.rs`.

```
cargo run -- \
  --tx-version 2 \
  --segwit \
  --input 8fb0d07bb3766421bff2d908b70e5de818e4d85a436ea3606310c1052b0dc821:1 \
  --output 69886:0014a632c1fff47af29f8c81dc4c6e91eb49a116c12b \
  --output 29442:00149831122b93d21715c70db626ccc844d3c21f9687 \
  --witness 3045022100f8704a3e7d55d4b5ee448cc6365caeffa42c2b00f74a37726d4fa3c11982e3e502203591c4a4bde9200281755ae5a8759116ce6e0cc7f5d30cf0eeb5b2b74f74bab301,029cbb1e568de08f469a8751aa2000331f130ca92ad49012d9cececaf6f8eb2358
```

```
0200000000010121c80d2b05c1106360a36e435ad8e418e85d0eb708d9f2bf216476b37bd0b08f
0100000000ffffffff02fe10010000000000160014a632c1fff47af29f8c81dc4c6e91eb49a116
c12b02730000000000001600149831122b93d21715c70db626ccc844d3c21f9687024830450221
00f8704a3e7d55d4b5ee448cc6365caeffa42c2b00f74a37726d4fa3c11982e3e502203591c4a4
bde9200281755ae5a8759116ce6e0cc7f5d30cf0eeb5b2b74f74bab30121029cbb1e568de08f46
9a8751aa2000331f130ca92ad49012d9cececaf6f8eb235800000000

Transaction size: 223 bytes
```

Note the serialized txid `21c80d2b...` is the reverse of the `8fb0d07b...` that was supplied.

### 3. Multiple inputs and outputs, mixed witness

Two inputs where only the first is a SegWit spend, three outputs, and a non-zero locktime.

```
cargo run -- \
  --tx-version 2 \
  --segwit \
  --input 8fb0d07bb3766421bff2d908b70e5de818e4d85a436ea3606310c1052b0dc821:0 \
  --input 3c1804567a336c3944e30b3c2593970bfcbf5b15a40f4fc6b626a360ee0507f2:1:'':0xfffffffd \
  --output 50000:0014a632c1fff47af29f8c81dc4c6e91eb49a116c12b \
  --output 25000:00149831122b93d21715c70db626ccc844d3c21f9687 \
  --output 10000:76a914e8df018c7e326cc253faac7e46cdc51e68542c4288ac \
  --witness 3045022100f8704a3e7d55d4b5ee448cc6365caeffa42c2b00f74a37726d4fa3c11982e3e502203591c4a4bde9200281755ae5a8759116ce6e0cc7f5d30cf0eeb5b2b74f74bab301,029cbb1e568de08f469a8751aa2000331f130ca92ad49012d9cececaf6f8eb2358 \
  --witness '' \
  --locktime 850000
```

```
0200000000010221c80d2b05c1106360a36e435ad8e418e85d0eb708d9f2bf216476b37bd0b08f
0000000000fffffffff20705ee60a326b6c64f0fa4155bbffc0b9793253c0be344396c337a5604
183c0100000000fdffffff0350c3000000000000160014a632c1fff47af29f8c81dc4c6e91eb49
a116c12ba8610000000000001600149831122b93d21715c70db626ccc844d3c21f968710270000
000000001976a914e8df018c7e326cc253faac7e46cdc51e68542c4288ac02483045022100f870
4a3e7d55d4b5ee448cc6365caeffa42c2b00f74a37726d4fa3c11982e3e502203591c4a4bde920
0281755ae5a8759116ce6e0cc7f5d30cf0eeb5b2b74f74bab30121029cbb1e568de08f469a8751
aa2000331f130ca92ad49012d9cececaf6f8eb23580050f80c00

Transaction size: 299 bytes
```

The second input carries an empty witness stack, serialized as the single byte `00`.

## Verifying the output

The sibling `decodetrx` crate parses the hex back into fields:

```
cd ../decodetrx
cargo run -- <hex from serializeTrx>
```

For example 2 it reports the input txid as `8fb0d07bb3766421bff2d908b70e5de818e4d85a436ea3606310c1052b0dc821`, matching what was typed in, which confirms the byte-order handling on both sides.

## Layout

| File | Contains |
|---|---|
| `src/main.rs` | Wires the pieces together and prints the result |
| `src/cli.rs` | Argument definitions, parsing, and validation |
| `src/transaction.rs` | `Transaction`, `TxInput`, `TxOutput` |
| `src/serialize.rs` | `serialize_transaction` and `encode_varint` |
| `src/hex.rs` | `hex_to_bytes` and `bytes_to_hex` |

The serialization logic itself is unchanged by the refactor. A test in `src/serialize.rs` builds the old hardcoded transaction from raw bytes and asserts it still produces the exact hex the pre-refactor program printed.

## Tests

```
cargo test
```

14 tests covering CompactSize boundaries, hex parsing and its rejections, txid reversal, optional input fields, witness attachment order, the validation rules, and the pre-refactor output check.

