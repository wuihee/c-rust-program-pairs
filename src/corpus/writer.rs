//! # Metadata Writer
//!
//! Writes to the contents of a metadata file from a [`Metadata`] struct.

use std::{fs, path::Path};

use crate::corpus::{
    errors::WriterError,
    metadata_structs::{
        CRustProgramPairSchema, DocumentationUrl, FeatureRelationship, IndividualProgram,
        IndividualProgramPair, ProgramDescription, ProgramName, RepositoryUrl, SourcePaths,
        TranslationTools,
    },
    schema::{Features, Metadata, Program, ProgramPair},
};

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
pub fn write_metadata(file_path: &Path, metadata: &Metadata) -> Result<(), WriterError> {
    // Convert our simplified Metadata struct to the auto-generated schema
    let schema = convert_metadata_to_schema(metadata)?;

    let metadata_json =
        serde_json::to_string_pretty(&schema).map_err(|error| WriterError::Serialize { error })?;

    fs::write(file_path, metadata_json).map_err(|error| WriterError::IoWrite {
        path: file_path.to_path_buf(),
        error,
    })?;

    Ok(())
}

/// Converts our internal `Metadata` struct to the auto-generated `CRustProgramPairSchema`.
fn convert_metadata_to_schema(metadata: &Metadata) -> Result<CRustProgramPairSchema, WriterError> {
    let pairs: Vec<IndividualProgramPair> = metadata
        .pairs
        .iter()
        .map(convert_program_pair)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(CRustProgramPairSchema::IndividualPairsMetadata { pairs })
}

/// Converts a `ProgramPair` to an `IndividualProgramPair`.
fn convert_program_pair(pair: &ProgramPair) -> Result<IndividualProgramPair, WriterError> {
    Ok(IndividualProgramPair {
        program_name: ProgramName::try_from(pair.program_name.clone()).map_err(|e| {
            WriterError::Conversion {
                message: format!("Invalid program name '{}': {}", pair.program_name, e),
            }
        })?,
        program_description: ProgramDescription::from(pair.program_description.clone()),
        translation_tools: TranslationTools::from(pair.translation_tools.clone()),
        feature_relationship: convert_features(&pair.feature_relationship),
        c_program: convert_program(&pair.c_program),
        rust_program: convert_program(&pair.rust_program),
    })
}

/// Converts a `Program` to an `IndividualProgram`.
fn convert_program(program: &Program) -> IndividualProgram {
    IndividualProgram {
        documentation_url: DocumentationUrl::from(program.documentation_url.clone()),
        repository_url: RepositoryUrl::from(program.repository_url.clone()),
        source_paths: SourcePaths::from(program.source_paths.clone()),
    }
}

/// Converts our `Features` enum to the auto-generated `FeatureRelationship` enum.
fn convert_features(features: &Features) -> FeatureRelationship {
    match features {
        Features::RustSubsetOfC => FeatureRelationship::RustSubsetOfC,
        Features::RustEquivalentToC => FeatureRelationship::RustEquivalentToC,
        Features::RustSupersetOfC => FeatureRelationship::RustSupersetOfC,
        Features::Overlapping => FeatureRelationship::Overlapping,
    }
}
