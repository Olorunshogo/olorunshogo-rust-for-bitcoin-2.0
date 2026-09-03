# Week 5 Practical Assessment

**Author:** [Gideon Bature](https://github.com/GideonBature)  
**Platform:** Rust, `rust-bitcoin`, and `bip39`  
**Labs:** 10  
**Final score:** 100 points

For every lab:

1. Implement the four functions in the matching `src/labs/labXX_*.rs` file.
2. Pass the matching public test suite.
3. Run and inspect the completed work locally.
4. Complete the matching `submissions/lab_XX.md` evidence file.

Use only the published class mnemonic and disposable test keys. Never submit real
wallet recovery or signing material.

## Lab 01 — Identify address formats and networks

Recognize P2PKH, P2SH, P2WPKH, and P2TR prefixes. Parse real address strings with
`rust-bitcoin`, require the intended network, identify the script family, and return
the encoded scriptPubKey. Explain why a prefix is a clue but checksum and network
validation are still required.

## Lab 02 — Construct a legacy P2PKH lock

Start from a compressed public key. Derive its P2PKH address, build
`OP_DUP OP_HASH160 <pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG`, expose the committed
HASH160, and place the signature/public key in ScriptSig. Explain the difference
between key identity and spend authorization.

## Lab 03 — Wrap 2-of-3 multisig in P2SH

Build `2 <pub1> <pub2> <pub3> 3 OP_CHECKMULTISIG`, derive the P2SH address, construct
the outer scriptPubKey, and report both layers. Explain why matching the script hash
is necessary but does not itself satisfy the inner multisig rule.

## Lab 04 — Construct native P2WPKH

Derive a `bcrt1q...` address on regtest, build its version-0 20-byte witness program,
and model an empty ScriptSig with signature/public key in witness. Explain how this
differs from both P2PKH and P2SH-wrapped SegWit.

## Lab 05 — Build a compatibility map

Model four sender capabilities: Base58Check P2PKH, Base58Check P2SH, Bech32, and
Bech32m. Determine which senders can pay legacy P2PKH, wrapped SegWit, native SegWit,
and Taproot outputs. Explain why an older wallet may accept `3...` and reject
`bc1q...`, and why sending support differs from spending support.

## Lab 06 — Compare weight and fees

Implement BIP141 weight, round it to virtual size, calculate fees at a given feerate,
and reproduce the class comparison of approximately 226 vB for P2PKH versus 141 vB
for P2WPKH at 50 sat/vB. Explain why witness data is not simply removed or given one
flat whole-transaction discount.

## Lab 07 — Validate BIP39 recovery inputs

Validate the public 12-word test mnemonic, report entropy/checksum lengths, derive its
512-bit seed, and prove that a different optional passphrase creates a different
wallet. Explain why the checksum is error detection rather than encryption and why a
forgotten passphrase cannot be recovered from the mnemonic alone.

## Lab 08 — Derive a BIP32 key tree

Create a master xpriv, derive an xpriv/xpub pair at a complete path, derive a normal
public child from an xpub, and detect hardened steps. Explain the purpose of the chain
code, the watch-only use of xpubs, and why hardened children cannot be derived from a
parent xpub.

## Lab 09 — Decode a BIP44 path

Decode `m/44'/0'/2'/1/5`, explain each level, replace only the final index, and derive
the selected P2PKH address from the public test mnemonic. Explain zero-based account
and address indexes, hardened apostrophes, and the receive/change branch.

## Lab 10 — Prove deterministic recovery

Use one mnemonic and passphrase to derive index 0 on BIP44, BIP49, and BIP84 branches.
Record the regtest prefixes and script families, repeat the derivation from the same
inputs, and then change only the final index. Explain why identical recovery inputs
reproduce the same address and why restoring a wallet also depends on path/script
conventions.

## Marking

Each lab is worth ten points:

| Category | Points | Assessment |
|---|---:|---|
| Correct execution | 4 | One point per passing public Rust test |
| Commands and evidence | 3 | One point per completed required evidence section |
| Explanation | 3 | Instructor review for correctness and clarity |

GitHub Actions reports the automated portion out of 70. The instructor adds the
explanation portion out of 30.

