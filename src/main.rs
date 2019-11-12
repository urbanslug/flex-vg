use std::error::Error;
use std::fs::File;
use std::collections::HashMap;





fn main() -> Result<(), Box<dyn Error>> {
  
    // setup
    let filename = "ferris.cbor";
    let mut my_map: HashMap<String, u32> = HashMap::new();
    my_map.insert(String::from("age"), 32);

    // serialize
    let bin_file = File::create(filename)?;
    serde_cbor::to_writer(bin_file, &my_map)?;

    // deserialize
    let tux_file = File::open(filename)?;
    let tux: HashMap<String, u32> = serde_cbor::from_reader(tux_file)?;
    println!("{:?}", tux);

    Ok(())
}
