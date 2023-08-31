use crate::{
	common::PRNumber,
	docfile::DocFile,
	docfile_wrapper::DocFileWrapper,
	docfilename::DocFileName,
	error::{self, Result},
	schema::Schema,
};
use std::{
	collections::HashSet,
	path::{Path, PathBuf},
};

pub struct LoadCmd;

//TODO: remove std::process::exit and return proper errors

impl LoadCmd {
	/// Load PRDoc from one or more numbers
	pub(crate) fn load_numbers(
		numbers: Vec<PRNumber>,
		dir: &PathBuf,
	) -> error::Result<(bool, HashSet<DocFileWrapper>)> {
		let mut global_result = true;

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
							global_result &= false;
							None
						}
					},
					Err(e) => {
						global_result &= false;
						log::warn!("{e:?}");
						None
					},
				}
			})
			.collect();

		Ok((global_result, res))
	}

	/// Load one file and returns a wrapper
	pub(crate) fn load_file(file: &PathBuf) -> Result<DocFileWrapper> {
		let filename = DocFileName::try_from(file)?;
		let value = Schema::load(&file)?;
		let wrapper = DocFileWrapper::new(filename, value);
		Ok(wrapper)
	}

	pub(crate) fn load_list(
		file: &PathBuf,
		dir: &PathBuf,
	) -> Result<(bool, HashSet<DocFileWrapper>)> {
		let mut global_result = true;

		let numbers: Vec<PRNumber> = std::fs::read_to_string(file)
			.unwrap()
			.lines()
			.map(|line| {
				let num = line.parse::<PRNumber>();
				if num.is_err() {
					global_result &= false;
				}
				num.expect("A list file should only contain numbers")
			})
			.collect();
		let (r, wrapper) = Self::load_numbers(numbers, dir).unwrap();
		global_result &= r;
		Ok((global_result, wrapper))
	}

	pub(crate) fn load_from_folder(dir: &PathBuf) -> Result<(bool, HashSet<DocFileWrapper>)> {
		let res = DocFile::find(dir, false);
		let mut global_result = true;

		let wrapper = res
			.filter_map(|file| {
				let filename_maybe = DocFileName::try_from(&file);

				if let Ok(filename) = filename_maybe {
					// todo: DEDUP that
					let yaml = Schema::load(&file);
					if let Ok(value) = yaml {
						let wrapper = DocFileWrapper::new(filename, value);

						global_result &= true;
						log::info!("OK  {}", file.display());
						Some(wrapper)
					} else {
						global_result &= false;
						log::warn!("ERR {}", file.display());
						None
					}
				} else {
					log::warn!("Invalid file {:?}", file.display());
					None
				}
			})
			.collect();

		Ok((global_result, wrapper))
	}

	pub fn run(
		dir: &PathBuf,
		file: Option<PathBuf>,
		numbers: Option<Vec<PRNumber>>,
		list: Option<PathBuf>,
		json: bool,
	) -> Result<Option<bool>> {
		log::debug!("Loading from directory {}", dir.display());

		let (global_result, wrapper) = match (file, numbers, list) {
			(Some(f), None, None) => {
				let file_abs = if f.is_relative() { Path::new(&dir).join(&f) } else { f.clone() };
				let mut wrapper = HashSet::new();
				wrapper.insert(Self::load_file(&file_abs)?);

				(None, wrapper)
			},

			(None, Some(numbers), None) => {
				log::debug!("Loading numbers {:?}", numbers);
				let (global_result, wrapper) = Self::load_numbers(numbers, dir)?;
				(Some(global_result), wrapper)
			},

			(None, None, Some(list)) => {
				log::debug!("Loading list from {:?}", list);
				let (global_result, wrapper) = Self::load_list(&list, dir)?;
				(Some(global_result), wrapper)
			},

			(None, None, None) => {
				log::debug!("Loading all files in folder {}", dir.display());
				let (global_result, wrapper) = Self::load_from_folder(dir)?;
				(Some(global_result), wrapper)
			},

			_ => unreachable!(),
		};

		if json {
			println!("{}", serde_json::to_string_pretty(&wrapper).unwrap());
		} else {
			println!("{}", serde_yaml::to_string(&wrapper).unwrap());
		}

		Ok(global_result)
		// // FILE
		// if let Some(f) = file.clone() {
		// 	let file_abs = if f.is_relative() { Path::new(&dir).join(&f) } else { f.clone() };
		// 	let wrapper = Self::load_file(&file_abs)?;

		// 	if json {
		// 		println!("{}", serde_json::to_string_pretty(&wrapper).unwrap());
		// 	} else {
		// 		println!("{}", serde_yaml::to_string(&wrapper).unwrap());
		// 	}
		// 	return Ok(());
		// }

		// NUMBER(s)
		// if let Some(numbers) = numbers.clone() {
		// 	log::debug!("Loading numbers {:?}", numbers);
		// 	let wrapper = Self::load_numbers(numbers, dir).unwrap(); // todo
		// 	if json {
		// 		println!("{}", serde_json::to_string_pretty(&wrapper).unwrap());
		// 	} else {
		// 		println!("{}", serde_yaml::to_string(&wrapper).unwrap());
		// 	}
		// 	return Ok(());
		// }

		// LIST
		// if let Some(list) = list {
		// 	log::debug!("Loading list from {:?}", list);
		// 	let wrapper: HashSet<DocFileWrapper> = Self::load_list(&list, dir).unwrap(); // todo

		// 	// todo: extract the printing at the end
		// 	if json {
		// 		println!("{}", serde_json::to_string_pretty(&wrapper).unwrap());
		// 	} else {
		// 		println!("{}", serde_yaml::to_string(&wrapper).unwrap());
		// 	}
		// 	return Ok(());
		// }

		// ALL FROM FOLDER
		// todo: handle the dir case
		// if numbers.is_none() && file.is_none() {
		// 	log::debug!("Loading all files in folder {}", dir.display());
		// 	// todo: removw unwrap
		// 	let (mut global_result, wrapper) = Self::load_from_folder(dir).unwrap();

		// 	if json {
		// 		println!("{}", serde_json::to_string_pretty(&wrapper).unwrap());
		// 	} else {
		// 		println!("{}", serde_yaml::to_string(&wrapper).unwrap());
		// 	}

		// 	// if files.is_empty() {
		// 	// 	eprintln!("No valid file found in {}", dir.display());
		// 	// 	std::process::exit(exitcode::DATAERR);
		// 	// }

		// 	let output_str = if json {
		// 		serde_json::to_string_pretty(&wrapper).unwrap_or_else(|_| {
		// 			global_result &= false;
		// 			String::new()
		// 		})
		// 	} else {
		// 		serde_yaml::to_string(&wrapper).unwrap_or_else(|_| {
		// 			global_result &= false;
		// 			String::new()
		// 		})
		// 	};

		// 	println!("{output_str}");

		// 	if !global_result {
		// 		eprintln!("__________");
		// 		eprintln!("Some errors in {}", dir.display());
		// 	}

		// 	Ok(())

		// // println!("global_result = {:?}", global_result);
		// // End process with appropriate status
		// if global_result {
		// 	std::process::exit(exitcode::OK);
		// } else {
		// 	std::process::exit(exitcode::DATAERR);
		// }
		// } else {
		// 	unreachable!();
		// }
	}
}
