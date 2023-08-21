use regex::Regex;
use std::{
	ffi::OsString,
	fmt::Display,
	path::{Path, PathBuf},
	str::FromStr,
};

use crate::{common::PRNumber, title::Title};

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

	// pub fn find(number: PRNumber, title: Option<String>)
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
