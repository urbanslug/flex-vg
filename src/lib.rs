#![allow(dead_code, unused_imports, unused_variables, unused_imports)]
pub mod data_structures;
mod io;

pub use io::cli::start;
pub use io::variation::gen_variations;
pub use io::variation::handle_vcf;
pub use io::variation::open_vcf;
