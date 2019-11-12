use std::fs::{self, File};
use std::error::Error;
// Graph serialization
use std::collections::HashMap;
use serde_test::{Token, assert_tokens, assert_ser_tokens};


// Serialize with serde CBOR

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_me() {
        let mut my_map: HashMap<String, u32> = HashMap::new();

        // serialize and deserialize

        assert_tokens(&my_map, &[
            Token::Map {len: Some(0)},
            Token::MapEnd,
        ]);

        my_map.insert(String::from("age"), 32);

        assert_tokens(&my_map, &[
            Token::Map {len: Some(1)},

            Token::String("age"),
            Token::U32(32),

            Token::MapEnd,
        ]);
    }

    #[test]
    fn creates_and_saves_graph() {
        let mut my_map: HashMap<String, u32> = HashMap::new();
        my_map.insert(String::from("age"), 32);
        let filename = "ferris.cbor";

        serialize_graph(filename, &my_map);

        // deserialize
        let f = File::open(filename).unwrap();
        let my_deserialized_map: HashMap<String, u32> = serde_cbor::from_reader(f).unwrap();

        assert_eq!(my_deserialized_map, my_map);


        fs::remove_file(filename);
    }

}


fn serialize_graph(filename: &str, my_map: &HashMap<String, u32> ) -> Result<(), Box<dyn Error>> {
    // serialize
    let bin_file = File::create(filename)?;
    serde_cbor::to_writer(bin_file, my_map)?;


    Ok(())
}
