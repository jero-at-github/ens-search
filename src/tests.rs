#[cfg(test)]
mod tests {
    use crate::{display_vec, namehash};

    #[test]
    fn zero() {
        let result = namehash("".into());
        assert_eq!(format!("0x{:?}", result), format!("0x{:?}", [0; 32]));
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
