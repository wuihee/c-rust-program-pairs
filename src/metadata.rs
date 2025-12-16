//! # Metadata
//!
//! This module automates the process of finding source files for C programs
//! for metadata files.

use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
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
    let source_files: Vec<String> = Vec::new();
    let makefiles = find_file("Makefile.am", repository);
    let to_check = format!("{program_name}_SOURCES");

    for file in makefiles {
        let Ok(lines) = read_lines(&file) else {
            eprintln!("Failed to read {file:?}");
            continue;
        };

        let lines = lines.filter_map(Result::ok);
        let normalized = normalize_makefile(lines);
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

/// Returns an iterator over the lines of a file.
///
/// # Arguments
///
/// - `file_name`: The path to the file.
///
/// # Returns
///
/// An iterator yielding each line of the file.
fn read_lines<P>(file_name: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_name)?;
    Ok(BufReader::new(file).lines())
}

/// Normalizes Makefile line continuations.
///
/// Lines ending with `\` are concatenated with the following lines, producing
/// one logical line per continuation group.
///
/// # Arguments
///
/// - `line`: An iterator over raw lines from a Makefile.
///
/// # Returns
///
/// A vector of logical lines with continuation markers removed.
fn normalize_makefile<I>(lines: I) -> Vec<String>
where
    I: Iterator<Item = String>,
{
    let mut normalized = Vec::new();
    let mut continued_line = String::new();

    for line in lines {
        let trimmed = line.trim_end();

        if trimmed.ends_with('\\') {
            continued_line.push_str(trimmed.trim_end_matches('\\'));
        } else {
            continued_line.push_str(trimmed);
            normalized.push(continued_line.clone());
            continued_line.clear();
        }
    }

    if !continued_line.is_empty() {
        normalized.push(continued_line);
    }

    normalized
}
