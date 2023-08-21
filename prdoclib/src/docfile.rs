use serde_yaml::Value;
use std::path::PathBuf;

use crate::{common::PRNumber, docfilename::DocFileName, schema::Schema};

#[derive(Debug)]
pub struct DocFile {
	pub file: PathBuf,
	pub content: Value,
}

impl From<PathBuf> for DocFile {
	fn from(file: PathBuf) -> Self {
		let content = Self::load(&file).unwrap();
		Self { file, content }
	}
}

impl DocFile {
	pub fn new(n: PRNumber) -> Self {
		let filename = DocFileName::from(n);
		let file = PathBuf::from(filename);
		let content = Self::load(&file).unwrap();
		Self { file, content }
	}

	pub fn load(file: &PathBuf) -> crate::error::Result<Value> {
		Schema::load(file)
	}

	pub fn generate() -> String {
		let template = include_str!("../../template.prdoc");
		String::from(template)
	}

	pub fn find(dir: &PathBuf, valid_only: bool) -> impl Iterator<Item = PathBuf> {
		//todo: remove unwrap
		std::fs::read_dir(dir)
			.unwrap()
			.filter_map(|res| res.ok())
			// Map the directory entries to paths
			.map(|dir_entry| dir_entry.path())
			// Filter out all paths with extensions other than what we want
			.filter_map(|path| if path.extension().map_or(false, |ext| ext == "prdoc") { Some(path) } else { None })
			.filter_map(move |path| {
				// println!("path1 = {:?}", path);
				if !valid_only || DocFileName::is_valid(&path) {
					// println!("OK");
					Some(path)
				} else {
					// println!("NOK");
					None
				}
			})
			.filter_map(move |path| {
				// println!("path2 = {:?}", path);
				if !valid_only || Schema::check(&path) {
					Some(path)
				} else {
					None
				}
			})
	}
}

#[cfg(test)]
mod test_doc_file_name {
	use super::*;

	#[test]
	fn test_mix() {
		assert_eq!(String::from("pr_123.prdoc"), DocFileName::from(123).to_string());
	}

	#[test]
	fn test_valid_names() {
		assert!(DocFileName::is_valid("pr_0.prdoc"));
		assert!(DocFileName::is_valid("pr_123.prdoc"));
		assert!(DocFileName::is_valid("pr_123_foo.prdoc"));
		assert!(DocFileName::is_valid("PR_123.prdoc"));

		assert!(!DocFileName::is_valid("PR_123.txt"));
		assert!(!DocFileName::is_valid("PR_ABC.txt"));
		assert!(!DocFileName::is_valid("1234.prdoc"));
	}
}
