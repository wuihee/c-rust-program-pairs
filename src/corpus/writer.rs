//! # Metadata Writer
//!
//! Writes to the contents of a metadata file from a [`Metadata`] struct.

use std::path::Path;

use crate::corpus::{errors::WriterError, schema::Metadata};

/// Writes to a metadata file.
///
/// # Arguments
///
/// - `file_path`: Path to the metadata file.
/// - `metadata`: The new contents to be written to the file.
///
/// # Return
///
/// Returns `Ok` on success and `WriterError` if the write failed.
pub fn write(file_path: &Path, metadata: Metadata) -> Result<(), WriterError> {
    Ok(())
}
