use crate::{common::PRNumber, doc_filename::DocFileName, docfile::DocFile, title::Title};
use log::debug;
use std::path::{Path, PathBuf};

pub struct GenerateCmd;

impl GenerateCmd {
	pub fn run(
		save: bool,
		number: PRNumber,
		title: Option<Title>,
		output_dir: &PathBuf,
	) -> std::io::Result<()> {
		// generate doc
		let template = DocFile::generate();

		// print to stdout or save to file
		if !save {
			debug!("Printing to stdout only, use --save to save to a file");
			println!("{}", &template);
			Ok(())
		} else {
			// generate filename based on number and title
			let filename: PathBuf = DocFileName::new(number, title).into();
			let output_file = Path::new(&output_dir).join(filename);
			debug!("template = {:?}", &template);
			debug!("output_file = {:?}", &output_file);
			std::fs::write(output_file, template)
		}
	}
}
