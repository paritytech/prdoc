//! A PRDoc is mostly identified by its `PRNumber` but it can also be identified by its filename.

use crate::{common::PRNumber, doc_filename::DocFileName};
use serde::Serialize;
use std::path::PathBuf;

/// This enum defines the initial input used to find a PRDoc.
#[derive(Debug, PartialEq, Eq, Hash, Serialize)]
pub enum PRDocSource {
	/// Filename of the PRDoc
	File(PathBuf),

	/// PR number of the PRDoc
	Number(PRNumber),

	/// Filename and PR number of the PRDoc
	Both(PathBuf, PRNumber),
}

impl From<PathBuf> for PRDocSource {
	fn from(file: PathBuf) -> Self {
		Self::File(file)
	}
}

impl From<&PathBuf> for PRDocSource {
	fn from(file: &PathBuf) -> Self {
		Self::File(file.clone())
	}
}

impl From<PRNumber> for PRDocSource {
	fn from(number: PRNumber) -> Self {
		Self::Number(number)
	}
}

impl From<(PathBuf, PRNumber)> for PRDocSource {
	fn from((file, number): (PathBuf, PRNumber)) -> Self {
		Self::Both(file, number)
	}
}

impl From<DocFileName> for PRDocSource {
	fn from(file: DocFileName) -> Self {
		Self::File(file.into())
	}
}

impl From<&PRDocSource> for PRNumber {
	fn from(source: &PRDocSource) -> Self {
		match source {
			PRDocSource::File(file) => {
				let file_maybe = DocFileName::try_from(file);
				match file_maybe {
					Ok(file) => file.number,
					Err(_e) => {
						log::warn!("No PR number could be found in {:#?}", file);
						0
					},
				}
			},
			PRDocSource::Number(number) => *number,
			PRDocSource::Both(_file, number) => *number,
		}
	}
}
