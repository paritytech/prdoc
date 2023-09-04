//! A wrapper to serialize both content and filename

use serde::Serialize;
use serde_yaml::Value;

use crate::doc_filename::DocFileName;

/// This wrapper is used to extend a docfile with "external" data
/// such as information we can find in the filename itself, that is:
/// - pr number
/// - title

#[derive(Debug, Serialize, Hash, PartialEq, Eq)]
pub struct DocFileWrapper {
	pub filename: DocFileName,
	pub content: Value,
}

impl DocFileWrapper {
	pub fn new(filename: DocFileName, content: Value) -> Self {
		Self { filename, content }
	}
}
