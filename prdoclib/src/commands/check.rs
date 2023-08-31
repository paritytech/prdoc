use crate::{common::PRNumber, docfile::DocFile, docfilename::DocFileName, schema::Schema};
use log::debug;
use std::path::{Path, PathBuf};

pub struct CheckCmd;

//TODO: remove std::process::exit and return proper errors

impl CheckCmd {
	pub fn run(directory: &PathBuf, file: Option<PathBuf>, number: Option<PRNumber>) {
		let dir = directory;
		debug!("Checking directory {}", dir.display());

		if let Some(file) = file {
			let file = if file.is_relative() { Path::new(&dir).join(&file) } else { file.clone() };
			debug!("Checking file {}", file.display());

			// todo: DEDUP that
			let result = Schema::check(&file);
			if result {
				println!("OK  {}", file.display());
				std::process::exit(exitcode::OK);
			} else {
				eprintln!("ERR {}", file.display());
				std::process::exit(exitcode::DATAERR);
			}
		}

		if let Some(number) = number {
			debug!("Checking PR #{}", number);
			let search = DocFileName::find(number, None, dir);

			let file = match search {
				Ok(f) => f,
				Err(e) => {
					eprintln!("e = {:?}", e);
					std::process::exit(exitcode::DATAERR)
				},
			};

			debug!("Checking file {}", file.display());

			// todo: DEDUP that
			let result = Schema::check(&file);
			if result {
				println!("OK  {}", file.display());
				std::process::exit(exitcode::OK);
			} else {
				eprintln!("ERR {}", file.display());
				std::process::exit(exitcode::DATAERR);
			}
		}

		if number.is_none() && file.is_none() {
			debug!("Checking all files in folder {}", dir.display());
			let res = DocFile::find(dir, false);
			let mut global_result = true;

			let mut count = 0;

			res.for_each(|file| {
				count += 1;

				// todo: DEDUP that
				let result = Schema::check(&file);
				if result {
					global_result &= true;
					println!("OK  {}", file.display());
				} else {
					global_result &= false;
					eprintln!("ERR {}", file.display());
				}
			});

			if count == 0 {
				eprintln!("No valid file found in {}", dir.display());
				std::process::exit(exitcode::DATAERR);
			}

			if global_result {
				println!("All OK in {}", dir.display());
				std::process::exit(exitcode::OK);
			} else {
				eprintln!("__________");
				eprintln!("Some errors in {}", dir.display());
				// todo: show the issues
				std::process::exit(exitcode::DATAERR);
			}
		}
	}
}
