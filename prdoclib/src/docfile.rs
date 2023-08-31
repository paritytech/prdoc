use log::*;
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
		let template = include_str!("../template.prdoc");
		String::from(template)
	}

	pub fn find(dir: &PathBuf, valid_only: bool) -> impl Iterator<Item = PathBuf> {
		//todo: remove unwrap
		trace!("valid_only: {valid_only}");

		std::fs::read_dir(dir)
			.unwrap()
			.filter_map(|res| res.ok())
			// Map the directory entries to paths
			.map(|dir_entry| dir_entry.path())
			// Filter out all paths with extensions other than what we want
			.filter_map(|path| {
				if path.extension().map_or(false, |ext| ext == "prdoc") {
					Some(path)
				} else {
					None
				}
			})
			.filter_map(move |path| {
				if valid_only {
					let is_valid = DocFileName::is_valid(&path);
					trace!(
						"{}: filename {}",
						path.display(),
						if is_valid { " VALID " } else { "INVALID" }
					);
					if is_valid {
						Some(path)
					} else {
						None
					}
				} else {
					Some(path)
				}
			})
			.filter_map(move |path| {
				let schema_valid = Schema::check(&path);
				trace!(
					"{}: schema {}",
					path.display(),
					if schema_valid { " VALID " } else { "INVALID" }
				);

				if valid_only {
					if schema_valid {
						Some(path)
					} else {
						None
					}
				} else {
					Some(path)
				}
			})
	}
}
