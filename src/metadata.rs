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

    for makefile_path in makefiles {
        let files = get_source_files_from_makefile(repository, &makefile_path, program_name);
        println!("{files:?}")

        // for file in files {
        //     update_source_files(&source_files, file);
        // }
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
fn find_file<P>(file_name: &str, directory: P) -> Vec<PathBuf>
where
    P: AsRef<Path>,
{
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

/// Retrieves a list of source files for a program from a Makefile.
///
/// # Arguments
///
/// - `repository`: Path to the repository in `repository_clones`.
/// - `makefile_path`: Path to the Makefile to search.
/// - `program_name`: Name of the program to find source files.
///
/// # Returns
///
/// A `Vec` of paths to the source files for `program_name` as described in
/// the Makefile.
fn get_source_files_from_makefile(
    repository: &Path,
    makefile_path: &Path,
    program_name: &str,
) -> Vec<PathBuf> {
    let lines = match read_lines(makefile_path) {
        Ok(lines) => lines,
        Err(_) => return Vec::new(),
    };

    let sources_key = format!("{program_name}_SOURCES");

    normalize_makefile(lines.filter_map(Result::ok))
        .iter()
        .filter(|line| line.starts_with(&sources_key))
        .flat_map(|line| {
            line.split_whitespace()
                .skip(2)
                .flat_map(|file| find_file(file, repository))
                .map(|path| path.to_path_buf())
        })
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
