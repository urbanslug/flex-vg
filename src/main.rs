use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    flex_vg::start();

    Ok(())
}
