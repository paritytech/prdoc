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
	pub file: PathBuf,
	pub filename: DocFileName,
	pub content: Value,
}

impl DocFileWrapper {
	pub fn new(file: PathBuf, filename: DocFileName, content: Value) -> Self {
		let file = file.canonicalize().expect("Canonicalize works");
		Self { file, filename, content }
	}
}
