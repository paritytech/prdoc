use thiserror::Error;
use valico::json_schema::ValidationState;

/// Result type alias
pub type Result<T> = std::result::Result<T, PRdocLibError>;

/// Custom error
#[derive(Error, Debug)]
pub enum PRdocLibError {
	#[error("ValidationErrors {0:?}")]
	IO(std::io::Error),

	#[error("ValidationErrors {0:?}")]
	ValidationErrors(ValidationState),

	/// Unknown error
	#[error("Unknown error")]
	Unknown(),
}

impl From<std::io::Error> for PRdocLibError {
	fn from(e: std::io::Error) -> Self {
		PRdocLibError::IO(e)
	}
}
