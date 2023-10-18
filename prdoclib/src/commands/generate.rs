use crate::{
	common::PRNumber, doc_filename::DocFileName, docfile::DocFile, schema::PRDOC_DEFAULT_DIR, title::Title,
};
use std::path::{Path, PathBuf};

pub struct GenerateCmd;

impl GenerateCmd {
	fn get_out_dir(output_dir: Option<PathBuf>) -> PathBuf {
		if let Some(dir) = output_dir {
			dir
		} else {
			match project_root::get_project_root() {
				Ok(dir) => dir.join(PRDOC_DEFAULT_DIR),
				Err(e) => {
					eprint!(
						"Project root not found, falling back to the current folder: {e:?}"
					);
					PathBuf::from(".")
				},
			}
		}
	}

	pub fn run(
		save: bool,
		number: PRNumber,
		title: Option<Title>,
		output_dir: Option<PathBuf>,
	) -> std::io::Result<()> {
		// generate doc
		let template = DocFile::generate();

		// print to stdout or save to file
		if !save {
			log::debug!("Printing to stdout only, use --save to save to a file");
			println!("{}", &template);
			Ok(())
		} else {
			// generate filename based on number and title
			let filename: PathBuf = DocFileName::new(number, title).into();

			let out_dir = Self::get_out_dir(output_dir);
			log::debug!("Storing prdoc in {out_dir:?}");
			std::fs::create_dir_all(&out_dir).unwrap_or_else(|why| {
				println!("! {:?}", why.kind());
			});

			let output_file = Path::new(&out_dir).join(filename);
			log::debug!("output_file = {:?}", &output_file);
			std::fs::write(output_file, template)
		}
	}
}
