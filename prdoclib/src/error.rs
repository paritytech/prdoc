//! Custom errors

use std::{ffi::OsString, path::PathBuf};

use thiserror::Error;
use valico::json_schema::ValidationState;

use crate::common::PRNumber;

/// Result type alias
pub type Result<T> = std::result::Result<T, PRdocLibError>;

/// Custom error
#[derive(Error, Debug)]
pub enum PRdocLibError {
	#[error("ValidationErrors {0:?}")]
	IO(std::io::Error),

	#[error("ValidationErrors {0:?}")]
	ValidationErrors(ValidationState),

	#[error("PRDoc not found for number {0}")]
	NumberNotFound(PRNumber),

	#[error("The filename is not valid: {0}")]
	InvalidFilename(PathBuf),

	#[error("The config is not valid: {0}")]
	InvalidConfig(PathBuf),

	/// Unknown error
	#[error("Unknown error")]
	Unknown(),
}

impl From<std::io::Error> for PRdocLibError {
	fn from(e: std::io::Error) -> Self {
		PRdocLibError::IO(e)
	}
}
