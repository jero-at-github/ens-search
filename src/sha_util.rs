use sha3::{Digest, Keccak256};

fn sha_vec_to_hex(vec: Vec<u8>) -> String {
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

pub fn sha3_hex(name: String) -> String {
    sha_vec_to_hex(sha3(name.as_bytes()))
}

fn namehash(name: String) -> Vec<u8> {
    if name.is_empty() {
        [0; 32].to_vec()
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

pub fn namehash_hex(name: String) -> String {
    sha_vec_to_hex(namehash(name))
}
