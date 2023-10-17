use crate::docfile::DocFile;
use std::{env, path::PathBuf};

pub struct ScanCmd;

impl ScanCmd {
	pub fn run(directories: Vec<PathBuf>, all: bool) {
		let current_dir = env::current_dir().expect("Failed retrieving the current dir !");
		log::debug!("Current dir: {}", current_dir.display());
		directories.iter().for_each(|directory| {
			let res = DocFile::find(directory, !all);

			if let Ok(dir) = res {
				dir.for_each(|hit| {
					println!("{}", hit.display());
				});
			} else {
				eprint!("Invalid directory: {}", directory.display());
			}
		});
	}
}
