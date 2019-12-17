use sha2::{Digest, Sha256};
use std::array::TryFromSliceError;
use std::convert::TryInto;

// We want to return store hashes as 32 byte arrays instead of
// hex strings to be conservative with memory usage
fn sha256(msg: &[u8]) -> Result<[u8; 32], TryFromSliceError> {
    Sha256::digest(msg).as_slice().try_into()
}

pub fn gen_node_hash(seq: &str, pos: usize) -> Result<[u8; 32], TryFromSliceError> {
    let mut hasher = Sha256::new();
    hasher.input(seq.as_bytes());

    // TODO: what if we used to_le/be_bytes here?
    // Return the memory representation of this integer as a byte array
    let p = format!("+{}", pos);
    hasher.input(p.as_bytes());

    hasher.result().as_slice().try_into()
}

#[cfg(test)]
mod tests {

    use hex;

    use super::*;

    #[test]
    fn test_gen_correct_node_hash() {
        let txt = "Hello, world!";
        let b_txt = txt.to_owned() + "+23";
        let result_hash = sha256(b_txt.as_bytes()).unwrap();

        let other_hash = gen_node_hash(txt, 23).unwrap();
        assert_eq!(other_hash, result_hash);
    }

    #[test]
    fn test_gen_correct_hash() {
        let txt = "Hello, World!".as_bytes();
        let result_hash = sha256(txt).unwrap();
        let hex_str = hex::encode(result_hash);
        assert_eq!(
            "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f",
            hex_str
        );
    }
}
