use std::path::PathBuf;
use crate::docfile::DocFile;

pub struct ScanCmd;

impl ScanCmd {
	pub fn run(directory: &PathBuf, all: bool) {
		let res = DocFile::find(&directory, !all);
		res.for_each(|hit| {
			println!("{}", hit.display());
		});
	}
}
