use crate::{
	common::PRNumber, doc_filename::DocFileName, docfile::DocFile, error, schema::Schema,
	utils::get_numbers_from_file,
};
use log::debug;
use std::{
	collections::HashSet,
	path::{Path, PathBuf},
};

/// Implementation of the main [check](/prdoc::opts::CheckOpts) command of the cli.
pub struct CheckCmd;

//TODO: remove std::process::exit and return proper errors

impl CheckCmd {
	pub(crate) fn check_number(number: PRNumber, dir: &PathBuf) -> error::Result<(PathBuf, bool)> {
		let file = DocFileName::find(number, None, dir)?;
		Ok((file.clone(), Self::check_file(&file)))
	}

	pub(crate) fn check_file(file: &PathBuf) -> bool {
		Schema::check_file(&file)
	}

	pub(crate) fn check_numbers(
		numbers: Vec<PRNumber>,
		dir: &PathBuf,
	) -> error::Result<HashSet<(PRNumber, bool)>> {
		let res = numbers
			.iter()
			.map(|&number| {
				log::debug!("Checking PR #{}", number);

				let file_maybe = DocFileName::find(number, None, dir);

				match file_maybe {
					Ok(file) => {
						let yaml = Schema::load(&file);

						if let Ok(_value) = yaml {
							(number, true)
						} else {
							(number, false)
						}
					},
					Err(e) => {
						log::warn!("{e:?}");
						(number, false)
					},
				}
			})
			.collect();

		Ok(res)
	}

	pub(crate) fn check_list(
		file: &PathBuf,
		dir: &PathBuf,
	) -> error::Result<HashSet<(PRNumber, bool)>> {
		let extract_numbers = get_numbers_from_file(file)?;

		let numbers: Vec<PRNumber> =
			extract_numbers.iter().filter_map(|(_, _, n)| n.to_owned()).collect();

		let res = Self::check_numbers(numbers, dir).unwrap();
		Ok(res)
	}

	pub fn run(
		dir: &PathBuf,
		file: Option<PathBuf>,
		numbers: Option<Vec<PRNumber>>,
		list: Option<PathBuf>,
	) -> HashSet<(Option<PRNumber>, bool)> {
		debug!("Checking directory {}", dir.display());

		let result = match (file, numbers, list) {
			(Some(file), None, None) => {
				let file =
					if file.is_relative() { Path::new(&dir).join(&file) } else { file.clone() };
				debug!("Checking file {}", file.display());

				let value = Schema::load(&file);
				let number = DocFileName::try_from(&file);
				let result = if let Ok(_value) = value {
					if let Ok(n) = number {
						(Some(n.number), true)
					} else {
						(None, false)
					}
				} else if let Ok(n) = number {
					(Some(n.number), false)
				} else {
					(None, false)
				};
				let mut h_result = HashSet::new();
				h_result.insert(result);
				h_result
			},

			(None, Some(numbers), None) => {
				debug!("Checking PR #{:?}", numbers);

				numbers
					.iter()
					.map(|&number| match Self::check_number(number, dir) {
						Ok((_file, res)) => (Some(number), res),
						Err(_e) => (None, false),
					})
					.collect()
			},

			(None, None, Some(list)) => Self::check_list(&list, dir)
				.unwrap()
				.iter()
				.map(|(num, status)| (Some(*num), *status))
				.collect(),

			(None, None, None) => {
				debug!("Checking all files in folder {}", dir.display());
				let res = DocFile::find(dir, false);
				let mut global_result = true;

				let mut count = 0;

				res.for_each(|file| {
					count += 1;

					// todo: DEDUP that
					let result = Schema::check_file(&file);
					if result {
						global_result &= true;
						println!("OK  {}", file.display());
					} else {
						global_result &= false;
						eprintln!("ERR {}", file.display());
					}
				});

				if count == 0 {
					eprintln!("⚠️ No valid file found in {}", dir.display());
					std::process::exit(exitcode::DATAERR);
				}

				if global_result {
					println!("All OK in {}", dir.display());
					std::process::exit(exitcode::OK);
				} else {
					eprintln!("__________");
					eprintln!(
						"⚠️ There are errors with files in {}",
						std::fs::canonicalize(dir)
							.map(|p| p.display().to_string())
							.unwrap_or("?".to_string())
					);
					// todo: show the issues
					std::process::exit(exitcode::DATAERR);
				}
			},

			_ => unreachable!(),
		};

		result
	}
}
