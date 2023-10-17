use crate::docfile::DocFile;
use std::{env, path::PathBuf};

pub struct ScanCmd;

impl ScanCmd {
	pub fn run(directories: Vec<PathBuf>, all: bool) -> Vec<PathBuf> {
		let current_dir = env::current_dir().expect("Failed retrieving the current dir !");
		log::debug!("Current dir: {}", current_dir.display());

		directories
			.iter()
			.flat_map(|directory| {
				if let Ok(dir) = DocFile::find(directory, !all) {
					dir.collect()
				} else {
					eprint!("Invalid directory: {}", directory.display());
					vec![]
				}
			})
			.collect()
	}
}
