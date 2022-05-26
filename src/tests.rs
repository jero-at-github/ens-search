#[cfg(test)]
mod tests {
    use crate::sha_util::{namehash_hex, sha3_hex};

    #[test]
    fn zero() {
        assert_eq!(
            namehash_hex("".into()),
            "0x0000000000000000000000000000000000000000000000000000000000000000"
        );
    }

    #[test]
    fn eth() {
        assert_eq!(
            sha3_hex("eth".into()),
            "0x4f5b812789fc606be1b3b16908db13fc7a9adf7ca72641f84d75b47069d3d7f0"
        );
    }

    #[test]
    fn foo_eth() {
        assert_eq!(
            sha3_hex("foo".into()),
            "0x41b1a0649752af1b28b3dc29a1556eee781e4a4c3a1f7f53f90fa834de098c4d"
        );
    }
}
