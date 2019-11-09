# Blake2B-PoW

A Rust Library That Performs Proof of Work (Consensus Algorithm) using Blake2B. Useful for Blockchain-related projects.

## Usage

```rust
extern crate blake2b_pow;
use blake2b_pow::{mine,verify_nonce};

fn main() {
    let correct_nonce = mine(&[0x3Eu8;32], 0xffffffc000000000);

    let _is_valid = verify_nonce(&[0x3Eu8;32], 0xffffffc000000000, correct_nonce);
}
```
