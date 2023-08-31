use std::path::{Path, PathBuf};
use log::{debug, info};
use crate::{
	common::PRNumber, docfile::DocFile, docfile_wrapper::DocFileWrapper, docfilename::DocFileName, schema::Schema,
};

pub struct LoadCmd;

//TODO: remove std::process::exit and return proper errors

impl LoadCmd {
	pub fn run(dir: &PathBuf, file: Option<PathBuf>, number: Option<Vec<PRNumber>>, list: Option<PathBuf>, json: bool) {
		debug!("Checking directory {}", dir.display());

		if let Some(file) = file {
			let file = if file.is_relative() { Path::new(&dir).join(&file) } else { file.clone() };
			let filename_maybe = DocFileName::try_from(&file);

			if let Ok(filename) = filename_maybe {
				debug!("Checking file {}", file.display());

				// todo: DEDUP that
				let yaml = Schema::load(&file);
				if let Ok(value) = yaml {
					let wrapper = DocFileWrapper::new(filename, value);
					if json {
						println!("{}", serde_json::to_string_pretty(&wrapper).unwrap());
					} else {
						println!("{}", serde_yaml::to_string(&wrapper).unwrap());
					}
					std::process::exit(exitcode::OK);
				} else {
					eprintln!("Error: {}", file.display());
					std::process::exit(exitcode::DATAERR);
				}
			} else {
				eprintln!("Error: {}", file.display());
				std::process::exit(exitcode::DATAERR);
			}
		}

		// todo: handle the "number" case
		if let Some(numbers) = number.clone() {
			for number in numbers {
				debug!("Checking PR #{}", number);
				let search = DocFileName::find(number, None, &dir);

				let file_maybe = match search {
					Ok(f) => f,
					Err(e) => {
						eprintln!("e = {:?}", e);
						std::process::exit(exitcode::DATAERR)
					}
				};

				if let Some(file) = file_maybe {
					let filename_maybe = DocFileName::try_from(&file);

					debug!("Checking file {}", file.display());
					if let Ok(filename) = filename_maybe {
						// todo: DEDUP that
						let yaml = Schema::load(&file);
						if let Ok(value) = yaml {
							let wrapper = DocFileWrapper::new(filename, value);

							if json {
								println!("{}", serde_json::to_string_pretty(&wrapper).unwrap());
							} else {
								println!("{}", serde_yaml::to_string(&wrapper).unwrap());
							}
							std::process::exit(exitcode::OK);
						} else {
							eprintln!("Error: {}", file.display());
							std::process::exit(exitcode::DATAERR);
						}
					} else {
						eprintln!("No file found");
						std::process::exit(exitcode::DATAERR);
					}
				} else {
					eprintln!("Error with PR {:?}", number);
					std::process::exit(exitcode::DATAERR);
				}
			}
		}

		// todo: handle the dir case
		if number.is_none() && file.is_none() {
			debug!("Loading all files in folder {}", dir.display());
			let res = DocFile::find(&dir, false);
			let mut global_result = true;

			let mut count = 0;
			let mut files: Vec<DocFileWrapper> = Vec::new();

			res.for_each(|file| {
				let filename_maybe = DocFileName::try_from(&file);
				if let Ok(filename) = filename_maybe {
					count += 1;

					// todo: DEDUP that
					let yaml = Schema::load(&file);
					if let Ok(value) = yaml {
						let wrapper = DocFileWrapper::new(filename, value);

						global_result &= true;
						info!("OK  {}", file.display());
						files.push(wrapper);
					} else {
						global_result &= false;
						eprintln!("ERR {}", file.display());
					}
				} else {
					eprintln!("Invalid file {:?}", file.display());
					std::process::exit(exitcode::DATAERR);
				}
			});

			if files.is_empty() {
				eprintln!("No valid file found in {}", dir.display());
				std::process::exit(exitcode::DATAERR);
			}

			// Output errors if some issues occur
			if !global_result {
				eprintln!("__________");
				eprintln!("Some errors in {}", dir.display());
			}

			// Create a string we can output and flag an error if something goes wrong
			let output_str = if json {
				serde_json::to_string_pretty(&files).unwrap_or_else(|_| {
					global_result &= false;
					String::new()
				})
			} else {
				serde_yaml::to_string(&files).unwrap_or_else(|_| {
					global_result &= false;
					String::new()
				})
			};

			println!("{output_str}");

			// println!("global_result = {:?}", global_result);
			// End process with appropriate status
			if global_result {
				std::process::exit(exitcode::OK);
			} else {
				std::process::exit(exitcode::DATAERR);
			}
		}
	}
}
