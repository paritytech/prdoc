//! Custom errors

use std::path::PathBuf;

use thiserror::Error;
use valico::json_schema::ValidationState;

use crate::common::PRNumber;

/// Result type alias
pub type Result<T> = std::result::Result<T, PRdocLibError>;

/// Custom error
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum PRdocLibError {
	#[error("ValidationErrors {0:?}")]
	IO(std::io::Error),

	#[error("ValidationErrors {0:?}")]
	ValidationErrors(ValidationState),

	#[error("PRDoc not found for number {0}")]
	NumberNotFound(PRNumber),

	#[error("PRDoc file already exists: {0}")]
	FileAlreadyExists(PathBuf),

	#[error("The filename is not valid: {0}")]
	InvalidFilename(PathBuf),

	#[error("The config is not valid: {0}")]
	InvalidConfig(PathBuf),

	#[error("No valid config found")]
	MissingConfig,

	#[error("No valid file found in {0}")]
	NoValidFileFound(PathBuf),

	#[error("Some valid files in {0}")]
	SomeInvalidFiles(PathBuf),

	/// Unknown error
	#[error("Unknown error")]
	Unknown,
}

impl From<std::io::Error> for PRdocLibError {
	fn from(e: std::io::Error) -> Self {
		PRdocLibError::IO(e)
	}
}
