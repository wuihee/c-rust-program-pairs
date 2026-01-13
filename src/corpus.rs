//! # Corpus
//!
//! This module transforms schema files into strongly-typed Rust structs.

mod delete;
pub mod downloader;
pub mod errors;
mod metadata_structs;
pub mod parser;
pub mod schema;
pub mod utils;
pub mod writer;

pub use delete::delete;
pub use downloader::download_program_pairs;
pub use parser::parse;
pub use utils::get_repository_name;
pub use writer::write_metadata;
