//! A wrapper to serialize both content and filename

use crate::doc_filename::DocFileName;
use serde::Serialize;
use serde_yaml::Value;
use std::path::PathBuf;

/// This wrapper is used to extend a docfile with "external" data
/// such as information we can find in the filename itself, that is:
/// - pr number
/// - title

#[derive(Debug, Serialize, Hash, PartialEq, Eq)]
pub struct DocFileWrapper {
	/// The file path
	pub file: PathBuf,

	/// The filename
	pub doc_filename: DocFileName,

	/// The content of the PRDoc
	pub content: Value,
}

impl DocFileWrapper {
	/// Create a new wrapper
	pub fn new(file: PathBuf, filename: DocFileName, content: Value) -> Self {
		let file = file.canonicalize().expect("Canonicalize works");
		Self { file, doc_filename: filename, content }
	}
}
