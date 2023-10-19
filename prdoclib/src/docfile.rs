//! The content of a `prdoc` file

use log::*;
use serde_yaml::Value;
use std::{fs, path::PathBuf};

use crate::{common::PRNumber, doc_filename::DocFileName, error, schema::Schema};

/// Wrapper around filename and content of a `prdoc` file
#[derive(Debug)]
pub struct DocFile {
	/// The file path
	pub file: PathBuf,

	/// The content of the PRDoc
	pub content: Value,
}

impl From<PathBuf> for DocFile {
	fn from(file: PathBuf) -> Self {
		let content = Self::load(&file).unwrap();
		Self { file, content }
	}
}

impl DocFile {
	/// Load a `prdoc` file given its PR number
	pub fn load_from_number(n: PRNumber) -> Self {
		let filename = DocFileName::from(n);
		let file = PathBuf::from(filename);
		let content = Self::load(&file).unwrap();
		Self { file, content }
	}

	/// Attempt to load a `prdoc` file given its filename
	pub fn load(file: &PathBuf) -> crate::error::Result<Value> {
		Schema::load(file)
	}

	/// Generate a new PRDoc
	pub fn generate(file: PathBuf) -> error::Result<String> {
		let template_file = if file.is_absolute() {
			file
		} else {
			let repo_root = project_root::get_project_root().expect("We need to work in a repo");
			repo_root.join(file)
		};

		Ok(fs::read_to_string(template_file)?)
	}

	/// Returns an iterator if the `dir` was a valid directory or an error otherwise.
	pub fn find(
		dir: &PathBuf,
		valid_only: bool,
	) -> crate::error::Result<impl Iterator<Item = PathBuf>> {
		trace!("valid_only: {valid_only}");

		let res = std::fs::read_dir(dir)?
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
				let schema_valid = Schema::check_file(&path);
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
			});
		Ok(res)
	}
}
