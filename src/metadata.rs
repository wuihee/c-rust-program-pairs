//! # Metadata
//!
//! This module automates the process of afinding and updating source files
//! for programs for metadata files.

use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

use crate::corpus::{self, writer::write_metadata};

pub fn update_metadata_file(repository: &Path) -> Result<(), Box<dyn Error>> {
    let metadata_path = repository.join("metadata.json");
    let mut metadata = corpus::parse(repository)?;

    for pair in metadata.pairs.iter_mut() {
        pair.c_program.source_paths = get_c_source_files(&pair.program_name, repository)?
            .into_iter()
            .map(|f| f.to_str().unwrap().to_string())
            .collect();
    }

    // Write the updated metadata back to the file
    write_metadata(&metadata_path, &metadata)?;

    println!("Successfully updated metadata at: {}", metadata_path.display());

    Ok(())
}

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
) -> Result<HashSet<PathBuf>, Box<dyn Error>> {
    let mut source_files: HashSet<PathBuf> = HashSet::new();

    let makefiles: Vec<PathBuf> = ["Makefile.am", "local.mk", "Makemodule.am"]
        .into_iter()
        .flat_map(|f| find_file(f, repository))
        .collect();

    for makefile_path in makefiles {
        let makefile_sources =
            get_source_files_from_makefile(repository, &makefile_path, program_name);

        for path in makefile_sources {
            collect_source_files(repository, &mut source_files, &path)?;
        }
    }

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
        Err(_) => {
            eprintln!("Failed to read makefile");
            return Vec::new();
        }
    };

    let sources_key = format!("{program_name}_SOURCES");

    normalize_makefile(lines.filter_map(Result::ok))
        .iter()
        .filter(|line| line.contains(&sources_key))
        .flat_map(|line| get_paths_from_line(line, repository))
        .collect()
}

/// Helper function to turn a line 'program_SOURCES = f1.c f2.c ...' into
/// an iterator containing f1.c, f2.c, ...
///
/// # Arguments
///
/// - `line`: The line in a Makefile containing a program's source files.
/// - `repository`: Path to the program's repository in `repository_clones`.
///
/// # Returns
///
/// An interator containing the program's source file paths.
fn get_paths_from_line(line: &str, repository: &Path) -> impl Iterator<Item = PathBuf> {
    line.split_whitespace()
        .skip(2)
        .map(path_to_file_name)
        .flat_map(move |file| find_file(file, repository))
        .map(|path| path.to_path_buf())
}

/// Helper method which takes a path and returns the file name.
///
/// # Arguments
///
/// - `file`: Path to the file.
///
/// # Returns
///
/// The final file name in the path.
///
/// # Example
///
/// `path/to/file.txt` returns `file.txt`.
fn path_to_file_name(file: &str) -> &str {
    Path::new(file)
        .file_name()
        .and_then(|file_name| file_name.to_str())
        .unwrap()
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

/// Recursively collects all source files starting from a single .c or .h
/// file.
///
/// # Arguments
///
/// - `repository`: Path to the repository in `repository_clones`.
/// - `visited`: A set of source files continuously updated.
/// - `root`: The starting source file to search from.
///
/// # Returns
///
/// `Ok` on success or `Err` otherwise.
fn collect_source_files(
    repository: &Path,
    visited: &mut HashSet<PathBuf>,
    root: &Path,
) -> Result<(), Box<dyn Error>> {
    let relative_path = root.strip_prefix(repository)?.to_path_buf();

    if !visited.insert(relative_path) {
        return Ok(());
    }

    for line in read_lines(root)?.flatten() {
        let include = line
            .strip_prefix("#include \"")
            .and_then(|s| s.strip_suffix('"'));

        if let Some(file_name) = include {
            for path in find_file(file_name, repository) {
                collect_source_files(repository, visited, &path)?;
            }
        }
    }

    Ok(())
}
