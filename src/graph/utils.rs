// Hashing
use sha2::{Digest, Sha256};

// Local
//use crate::graph::types::{Node, Graph};

/// The keys of the hash table are SHA256 hashes of the concatenation of:
///  - the sequence
///  - a plus symbol(+)
///  - offset.
fn gen_node_id<'a>(seq: &'a str, offset: usize) -> String {
    let str_to_hash = format!("{}+{}", seq, offset);

    // TODO: is it a good idea to keep creating a new hasher?
    // or use https://docs.rs/sha2/0.8.0/sha2/trait.Digest.html#tymethod.result_reset
    let mut hasher: Sha256 = Digest::new();

    hasher.input(str_to_hash.as_bytes());
    let a = hasher.result();
    let hex_str = format!("{:x}", a);

    // TODO: not return a copy of the string
    hex_str.clone()
}
