use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UtilError {

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("{0}")]
    FormatError(#[from] FormatError),

    #[error("{0}")]
    ExtensionError(#[from] ExtensionError),

    #[error("Error converting to absolute path: {0}")]
    AbsolutePathError(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum FormatError {
    #[error("Invalid options format: {0}")]
    InvalidOptions(String),

    #[error("Invalid Coordinate string format")]
    InvalidCoordinateFormat(String),

    #[error("Error parsing coordinates: {0}")]
    ParseIntError(#[from] ParseIntError),
}

/// Errors related to inferring index paths or output formats from file extensions.
#[derive(Debug, Error)]
pub enum ExtensionError {
    #[error("Error determining index path: {0}")]
    IndexPathError(String),

    #[error("Error determining file type: {0}")]
    PathTypeError(String),
}