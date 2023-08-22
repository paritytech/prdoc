use regex::Regex;
use std::{
	ffi::OsString,
	fmt::Display,
	path::{Path, PathBuf},
	str::FromStr,
};

use crate::{
	common::PRNumber,
	error::{self},
	title::Title,
};

#[derive(Debug, PartialEq)]
pub struct DocFileName {
	pub number: PRNumber,
	pub title: Option<Title>,
}

impl DocFileName {
	pub fn filename(&self) -> OsString {
		if let Some(title) = &self.title {
			OsString::from(format!("pr_{}_{:?}.prdoc", self.number, title.to_string()))
		} else {
			OsString::from(format!("pr_{}.prdoc", self.number))
		}
	}

	pub fn new(number: PRNumber, title: Option<Title>) -> Self {
		Self { number, title }
	}

	fn get_regex() -> Regex {
		Regex::new(r"^pr_(?<number>\d+).*\.prdoc$").unwrap()
	}

	pub fn is_valid<P: AsRef<Path>>(filename: P) -> bool {
		let re = Self::get_regex();
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

	/// Search for a PR Doc in a given folder and matching the args
	pub fn find(number: PRNumber, title: Option<String>, directory: &PathBuf) -> error::Result<Option<PathBuf>> {
		if title.is_some() {
			todo!("Searching by Number + title is not implemented yet, needed ?");
		}

		// We search for matching patterns and capture the `number` group
		let re: Regex = Self::get_regex();

		let hit_maybe = std::fs::read_dir(directory)?.find_map(|entry| match entry {
			Ok(candidate) => {
				// First we exclude anything that is not a file
				let metadata = std::fs::metadata(candidate.path()).unwrap();
				if !metadata.is_file() {
					return None;
				}

				// Fetch the file name
				let fname = candidate.file_name();
				let filename = fname.to_str().unwrap_or_default();

				// We capture numbers first
				let number_capture = re.captures(filename).and_then(|cap| {
					cap.name("number").map(|n| {
						let s = n.as_str();
						let my_num: PRNumber = s.parse().unwrap();
						my_num
					})
				});

				// Then check if the number we got matches.
				// It is required to do this so we also find `pr_000...` when looking for
				// PR #0
				if number_capture.is_some_and(|value| value == number) {
					Some(PathBuf::from(&directory).join(filename))
				} else {
					None
				}
			}
			Err(_e) => None,
		});

		if let Some(hit) = hit_maybe {
			Ok(Some(hit))
		} else {
			Ok(None)
		}
	}
}

impl Display for DocFileName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.filename().to_str().expect("Our filename are valid path"))
	}
}

impl From<PRNumber> for DocFileName {
	fn from(n: PRNumber) -> Self {
		Self::new(n, None)
	}
}

impl From<DocFileName> for PathBuf {
	fn from(val: DocFileName) -> Self {
		PathBuf::from_str(&val.to_string()).expect("Our filename are valid path")
	}
}

#[cfg(test)]
mod test_doc_file_name {
	use super::*;

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

	#[test]
	fn test_mix() {
		assert_eq!(String::from("pr_123.prdoc"), DocFileName::from(123).to_string());
	}

	#[test]
	fn test_find() {
		assert_eq!(
			Some(PathBuf::from("../tests/data/pr_1234_some_test_minimal.prdoc")),
			DocFileName::find(1234, None, &PathBuf::from("../tests/data")).unwrap()
		);
	}
}
