use serde::Serialize;
use serde_yaml::Value;

use crate::docfilename::DocFileName;

/// This wrapper is used to extend a docfile with "external" data
/// such as information we can find in the filename itself, that is:
/// - pr number
/// - title

#[derive(Debug, Serialize, Hash)]
pub struct DocFileWrapper {
	pub filename: DocFileName,
	pub content: Value,
}

impl PartialEq for DocFileWrapper {
	fn eq(&self, other: &Self) -> bool {
		self.filename == other.filename
	}
}

impl Eq for DocFileWrapper {}


impl DocFileWrapper {
	pub fn new(filename: DocFileName, content: Value) -> Self {
		Self { filename, content }
	}
}
