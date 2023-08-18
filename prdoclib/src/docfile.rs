use std::{
	fmt::Display,
	path::{Path, PathBuf},
	str::FromStr,
};

use regex::Regex;
use serde_yaml::Value;

use crate::{PRNumber, Schema};

#[derive(Debug, PartialEq)]
pub struct DocFileName(String);

#[derive(Debug)]
pub struct DocFile {
	pub file: PathBuf,
	pub content: Value,
}

impl DocFileName {
	pub fn new(n: PRNumber, title: Option<&str>) -> Self {
		if let Some(title) = title {
			// todo: more cleanup would be good to ensure we have a valid OsString
			let cleaned_up_title = title;
			Self(format!("pr_{n}_{cleaned_up_title}.prdoc"))
		} else {
			Self(format!("pr_{n}.prdoc"))
		}
	}

	pub fn is_valid<P: AsRef<Path>>(filename: P) -> bool {
		let re = Regex::new(r"^pr_\d+.*.prdoc$").unwrap();
		let file_only = filename.as_ref().components().last();
		if let Some(file) = file_only {
			match file {
				std::path::Component::Prefix(_)
				| std::path::Component::RootDir
				| std::path::Component::CurDir
				| std::path::Component::ParentDir => false,
				std::path::Component::Normal(f) => re.is_match(&PathBuf::from(f).display().to_string().to_lowercase()),
			}
		} else {
			false
		}
	}
}

impl Display for DocFileName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.0)
	}
}

impl AsRef<Path> for DocFileName {
	fn as_ref(&self) -> &Path {
		&Path::new(&self.0)
	}
}

impl From<PRNumber> for DocFileName {
	fn from(n: PRNumber) -> Self {
		Self::new(n, None)
	}
}

impl Into<PathBuf> for DocFileName {
	fn into(self) -> PathBuf {
		PathBuf::from_str(&self.0).unwrap()
	}
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
		let file = PathBuf::from(filename.as_ref());
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
		let files_iter = std::fs::read_dir(dir)
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
			});

		files_iter
	}
}

#[cfg(test)]
mod test_doc_file_name {
	use super::*;

	#[test]
	fn test_mix() {
		assert_eq!(DocFileName(String::from("pr_123.prdoc")), DocFileName::from(123));
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
