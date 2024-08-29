//! Implementation of the scan command.
//!
//! The scan command searches for files that could
//! potentially be PRDOC files. It does not check the validity of the files and the scanning is
//! solely done based on the filenames

use crate::{docfile::DocFile, schema::Schema};
use std::{env, path::PathBuf};

/// Wrapper to the scan command
pub struct ScanCmd;

impl ScanCmd {
	/// Run of the scan command
	pub fn run(schema: Schema, directories: Vec<PathBuf>, all: bool) -> Vec<PathBuf> {
		let current_dir = env::current_dir().expect("Failed retrieving the current dir !");
		log::debug!("Current dir: {}", current_dir.display());

		directories
			.iter()
			.flat_map(|directory| {
				if let Ok(dir) = DocFile::find(schema.clone(), directory, !all) {
					dir.collect()
				} else {
					eprint!("Invalid directory: {}", directory.display());
					vec![]
				}
			})
			.collect()
	}
}
