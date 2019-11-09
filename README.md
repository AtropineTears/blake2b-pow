# Blake2b-PoW

A Rust Library For Nano's Proof of Work Consensus Algorithm That Uses Blake2B.

## Usage

```rust
extern crate blake2b_pow;
use blake2b_pow::{mine,verify_nonce};

fn main() {
    let correct_nonce = mine(&[0x3Eu8;32], 0xffffffc000000000);

    let _is_valid = verify_nonce(&[0x3Eu8;32], 0xffffffc000000000, correct_nonce);
}
```
