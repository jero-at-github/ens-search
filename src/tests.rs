#[cfg(test)]
mod tests {
    use ens_domains::domain_to_hash;

    #[test]
    fn zero() {
        assert_eq!(
            domain_to_hash("".into()),
            "0x0000000000000000000000000000000000000000000000000000000000000000"
        );
    }

    #[test]
    fn eth() {
        assert_eq!(
            domain_to_hash("eth".into()),
            "0x93cdeb708b7545dc668eb9280176169d1c33cfd8ed6f04690a0bcc88a93fc4ae"
        );
    }

    #[test]
    fn foo_eth() {
        assert_eq!(
            domain_to_hash("foo.eth".into()),
            "0xde9b09fd7c5f901e23a3f19fecc54828e9c848539801e86591bd9801b019f84f"
        );
    }
}
