//! Implementation of the generate command. This command generates a new PRDoc file.

use crate::{
	common::PRNumber,
	doc_filename::DocFileName,
	docfile::DocFile,
	error::{self, PRdocLibError},
	schema::PRDOC_DEFAULT_DIR,
	title::Title,
	utils::*,
};
use std::path::{Path, PathBuf};

/// Wrapper to the generate command
pub struct GenerateCmd;

impl GenerateCmd {
	fn get_output_dir(output_dir: Option<PathBuf>) -> PathBuf {
		if let Some(dir) = output_dir {
			dir
		} else {
			match get_project_root() {
				Ok(dir) => dir.join(PRDOC_DEFAULT_DIR),
				Err(e) => {
					eprint!("Project root not found, falling back to the current folder: {e:?}");
					PathBuf::from(".")
				},
			}
		}
	}

	/// Run of the generate command
	pub fn run(
		dry_run: bool,
		number: PRNumber,
		title: Option<Title>,
		output_dir: Option<PathBuf>,
		template: PathBuf,
	) -> error::Result<()> {
		let template = DocFile::generate(template)?;

		if dry_run {
			// print to stdout or save to file
			log::debug!("Printing to stdout only due to --dry-run");
			println!("{}", &template);
			Ok(())
		} else {
			// generate filename based on number and title
			let filename: PathBuf = DocFileName::new(number, title).into();
			let output_dir = Self::get_output_dir(output_dir);
			log::debug!("Storing prdoc in {output_dir:?}");
			std::fs::create_dir_all(&output_dir).unwrap_or_else(|why| {
				println!("! {:?}", why.kind());
			});

			let output_file = Path::new(&output_dir).join(filename);
			log::debug!("output_file = {:?}", &output_file);

			if !output_file.exists() {
				std::fs::write(output_file, template).map_err(PRdocLibError::IO)
			} else {
				Err(PRdocLibError::FileAlreadyExists(output_file.clone()))
			}
		}
	}
}
