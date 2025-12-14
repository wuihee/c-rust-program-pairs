//! # Metadata
//!
//! This module automates the process of finding source files for C programs
//! for metadata files.

use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

/// Get a list of .c and .h source files for a C program.
///
/// # Arguments
///
/// - `program_name`: The name of the C program.
/// - `repository`: The path of the repository in `repository_clones` to
///                 search; this requires the repository to be downloaded.
///
/// # Returns
///
/// A `Vector` containing the path to all .c and .h source files, relative to
/// the path of the repository.
pub fn get_c_source_files(
    program_name: &str,
    repository: &Path,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut source_files: Vec<String> = Vec::new();
    let makefiles = find_file("Makefile.am", repository);

    for makefile in makefiles {
        let contents = fs::read_to_string(makefile)?;
        let to_search_for = format!("{program_name}_SOURCES");
        let index = contents.find(&to_search_for);
    }

    // For each file in the repository, if it is a makefile.am, search it.
    // Find the string that matches {program_name}_SOURCES.
    // This should return a space-separated line of .c programs.
    // For each of these c programs, find it in the repository, and
    // recursively search for all other dependencies.

    Ok(source_files)
}

/// Find a list of files in a directory.
///
/// # Arguments
///
/// - `file_name`: The name of the file to find.
/// - `directory`: The directory to search in.
///
/// # Returns
///
/// A `Vector` containing `PathBuf`s of all file matches.
fn find_file(file_name: &str, directory: &Path) -> Vec<PathBuf> {
    WalkDir::new(directory)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .file_name()
                .to_str()
                .map(|name| name == file_name)
                .unwrap_or(false)
        })
        .map(|entry| entry.path().to_path_buf())
        .collect()
}
