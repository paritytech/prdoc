use crate::{
	common::PRNumber,
	docfile::DocFile,
	docfile_wrapper::DocFileWrapper,
	docfilename::DocFileName,
	error::{self, Result},
	schema::Schema,
};
use std::{path::{Path, PathBuf}, collections::HashSet};

pub struct LoadCmd;

//TODO: remove std::process::exit and return proper errors

impl LoadCmd {
	/// Load PRDoc from one or more numbers
	pub(crate) fn load_numbers(
		numbers: Vec<PRNumber>,
		dir: &PathBuf,
	) -> error::Result<HashSet<DocFileWrapper>> {
		let res = numbers
			.iter()
			.filter_map(|&number| {
				log::debug!("Loading PR #{}", number);

				let file_maybe = DocFileName::find(number, None, dir);

				match file_maybe {
					Ok(file) => {
						let filename = DocFileName::try_from(&file)
							.expect("If we found a file, it should be valid");

						let yaml = Schema::load(&file);

						if let Ok(value) = yaml {
							Some(DocFileWrapper::new(filename, value))
						} else {
							None
						}
					},
					Err(e) => {
						log::warn!("{e:?}");
						None
					},
				}
			})
			.collect();

		// println!("res = {:?}", res);

		Ok(res)
	}

	/// Load one file and returns a wrapper
	pub(crate) fn load_file(file: &PathBuf) -> Result<DocFileWrapper> {
		let filename = DocFileName::try_from(file)?;
		let value = Schema::load(&file)?;
		let wrapper = DocFileWrapper::new(filename, value);
		Ok(wrapper)
	}

	pub fn run(
		dir: &PathBuf,
		file: Option<PathBuf>,
		numbers: Option<Vec<PRNumber>>,
		list: Option<PathBuf>,
		json: bool,
	) -> Result<()> {
		log::debug!("Loading from directory {}", dir.display());

		// FILE
		if let Some(f) = file.clone() {
			let file_abs = if f.is_relative() { Path::new(&dir).join(&f) } else { f.clone() };
			let wrapper = Self::load_file(&file_abs)?;

			if json {
				println!("{}", serde_json::to_string_pretty(&wrapper).unwrap());
			} else {
				println!("{}", serde_yaml::to_string(&wrapper).unwrap());
			}
			return Ok(());
		}

		// NUMBER(s)
		if let Some(numbers) = numbers.clone() {
			log::debug!("Loading numbers {:?}", numbers);
			let wrapper = Self::load_numbers(numbers, dir).unwrap(); // todo
			if json {
				println!("{}", serde_json::to_string_pretty(&wrapper).unwrap());
			} else {
				println!("{}", serde_yaml::to_string(&wrapper).unwrap());
			}
			return Ok(());
		}

		// LIST
		if let Some(_list) = list {
			todo!();
			return Ok(());
		}

		// ALL FROM FOLDER
		// todo: handle the dir case
		if numbers.is_none() && file.is_none() {
			log::debug!("Loading all files in folder {}", dir.display());
			let res = DocFile::find(dir, false);
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
						log::info!("OK  {}", file.display());
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
		} else {
			unreachable!();
		}
	}
}
