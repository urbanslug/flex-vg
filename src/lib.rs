#![allow(dead_code, unused_variables)]

//! flex_vg is a [variation graph] tool written in rust which aims for compatibility with [libhandlegraph].
//!
//! [variation graph]: https://blog.urbanslug.com/posts/2019-06-22-Introduction-to-Variation-Graphs.html
//! [libhandlegraph]: https://github.com/vgteam/libhandlegraph
//!
//! # Examples
//!
//! ```rust
//! // Real example coming up
//! assert_eq!(3,3);
//! ```

mod io;

pub mod graph;

pub use io::cli::start;
