//! Custom errors

use std::{convert::Infallible, path::PathBuf};

use thiserror::Error;
use valico::json_schema::{SchemaError, ValidationState};

use crate::common::PRNumber;

/// Result type alias
pub type Result<T> = std::result::Result<T, PRdocLibError>;

/// Custom error
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum PRdocLibError {
	#[error("ValidationErrors {0:?}")]
	IO(std::io::Error),

	#[error("Serde JSON error {0:?}")]
	SerdeJsonError(serde_json::Error),

	#[error("Serde YAML error {0:?}")]
	SerdeYamlError(serde_yaml::Error),

	#[error("ValidationErrors {0:?}")]
	ValidationErrors(ValidationState),

	#[error("Could not find the PRdoc for Pull Request #{0}. Did you forget to create a PRDoc?")]
	NumberNotFound(PRNumber),

	#[error("PRDoc file already exists: {0}")]
	FileAlreadyExists(PathBuf),

	#[error("The filename is not valid: {0}")]
	InvalidFilename(PathBuf),

	#[error("The config is not valid: {0}")]
	InvalidConfig(PathBuf),

	#[error("No valid config found")]
	MissingConfig,

	#[error("Template file at {0} was not found")]
	MissingTemplateFile(PathBuf),

	#[error("No valid file found in {0}")]
	NoValidFileFound(PathBuf),

	#[error("Some valid files in {0}")]
	SomeInvalidFiles(PathBuf),

	#[error("Schema error with {0}")]
	SchemaError(SchemaError),

	// Unknown error
	#[error("Unknown error")]
	Unknown,
}

impl From<std::io::Error> for PRdocLibError {
	fn from(e: std::io::Error) -> Self {
		PRdocLibError::IO(e)
	}
}

impl From<serde_json::Error> for PRdocLibError {
	fn from(e: serde_json::Error) -> Self {
		PRdocLibError::SerdeJsonError(e)
	}
}

impl From<serde_yaml::Error> for PRdocLibError {
	fn from(e: serde_yaml::Error) -> Self {
		PRdocLibError::SerdeYamlError(e)
	}
}

impl From<SchemaError> for PRdocLibError {
	fn from(e: SchemaError) -> Self {
		PRdocLibError::SchemaError(e)
	}
}

impl From<Infallible> for PRdocLibError {
	fn from(_value: Infallible) -> Self {
		PRdocLibError::Unknown
	}
}
