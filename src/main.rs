// Reference:
// https://docs.ens.domains/contract-api-reference/name-processing
// https://eips.ethereum.org/EIPS/eip-137

use sha3::{Digest, Keccak256};
use std::str;

const ZERO: [u8; 32] = [0; 32];

fn main() {
    let r = namehash("eth".into());
    println!("{}", display_vec(r));
}

fn display_vec(vec: Vec<u8>) -> String {
    let result: String = vec
        .iter()
        .map(|x| format!("{:02x?}", x))
        .collect::<Vec<String>>()
        .join("");

    ["0x".into(), result].concat()
}

fn sha3(name: &[u8]) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(name);
    let hasher_fin = hasher.finalize();
    let result: &[u8] = hasher_fin.as_slice();
    result.to_vec()
}

fn namehash(name: String) -> Vec<u8> {
    if name.is_empty() {
        ZERO.to_vec()
    } else {
        let split_vec: Vec<&str> = name.split('.').collect();
        let label = split_vec.get(0).unwrap().to_string();
        let remainder: String = split_vec.get(1).or(Some(&"")).unwrap().to_string();

        sha3(
            [namehash(remainder), sha3(label.as_bytes())]
                .concat()
                .as_slice(),
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::{display_vec, namehash, ZERO};

    #[test]
    fn zero() {
        let result = namehash("".into());
        assert_eq!(format!("0x{:?}", result), format!("0x{:?}", ZERO));
    }

    #[test]
    fn eth() {
        let result = namehash("eth".into());
        assert_eq!(
            display_vec(result),
            "0x93cdeb708b7545dc668eb9280176169d1c33cfd8ed6f04690a0bcc88a93fc4ae"
        );
    }

    #[test]
    fn mysite_swarm() {
        let result = namehash("foo.eth".into());
        assert_eq!(
            display_vec(result),
            "0xde9b09fd7c5f901e23a3f19fecc54828e9c848539801e86591bd9801b019f84f"
        );
    }
}
