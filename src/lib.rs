#![no_std]

use blake2_rfc::blake2b::blake2b;

mod params;

/// # Blake2B PoW
/// This rust library follows Nano's Proof of Work concept for their cryptocurrency. It uses the hash function Blake2B, and the input address, and keys it with a u64 nonce that is ever-increasing.
/// 
/// It takes as input an address, or previous block hash, as a u8 byte slice, and it also takes the threshold u64 value, with a higher value indicating greater difficulty.
/// 
/// It outputs the nonce that was either equal to or more than the threshold after the hashing process.
/// 
/// ## Default Value
/// * Threshold: 0xffffffc000000000
pub fn mine (address: &[u8], threshold: u64) -> u64 {
    let mut nonce: u64 = 0u64;
    
    loop {
        let w = blake2b(8, &nonce.to_be_bytes(), &address);
        
        if w.as_bytes() < &threshold.to_be_bytes() {
            nonce += 1u64;
        }
        else {
            return nonce
        }
    }
}

/// # Verify The Nonce
/// A Function to validate the nonce is correct and the output is above the threshold.
pub fn verify_nonce (address: &[u8], threshold: u64, nonce: u64) -> bool {
    let w = blake2b(8, &nonce.to_be_bytes(), &address);
    if w.as_bytes() >= &threshold.to_be_bytes() {
        return true
    }
    else {
        return false
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
