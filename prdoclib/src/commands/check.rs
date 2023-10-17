use super::utils::get_numbers_from_file;
use crate::{
	common::PRNumber,
	doc_filename::DocFileName,
	docfile::DocFile,
	error::{self},
	prdoc_source::PRDocSource,
	schema::Schema,
};
use std::{
	collections::HashSet,
	path::{Path, PathBuf},
};

/// Implementation of the main [check](/prdoc::opts::CheckOpts) command of the cli.
pub struct CheckCmd;

/// PRDoc are checked via a PR number or a file.
/// - When passing a file path, it may not result in a `PRNumber`.
/// - When passing a PRNumber, it also may not result in a file
pub type CheckResult = (PRDocSource, bool);

impl CheckCmd {
	pub(crate) fn check_numbers(
		numbers: Vec<PRNumber>,
		dir: &PathBuf,
	) -> error::Result<HashSet<CheckResult>> {
		log::debug!("Checking PR #{:?}", numbers);

		let res = numbers
			.iter()
			.map(|&number| {
				log::debug!("Checking PR #{}", number);

				let file_maybe = DocFileName::find(number, None, dir);

				match file_maybe {
					Ok(file) => {
						let yaml = Schema::load(&file);

						if let Ok(_value) = yaml {
							(number.into(), true)
						} else {
							(number.into(), false)
						}
					},
					Err(e) => {
						log::warn!("{e:?}");
						(number.into(), false)
					},
				}
			})
			.collect();

		Ok(res)
	}

	/// Check a PRDoc based on its number in a given folder.
	pub(crate) fn _check_number(number: PRNumber, dir: &PathBuf) -> error::Result<CheckResult> {
		let file = DocFileName::find(number, None, dir)?;
		Ok((file.clone().into(), Self::check_file(&file).1))
	}

	/// Check a specific file given its full path
	pub(crate) fn check_file(file: &PathBuf) -> CheckResult {
		// log::debug!("Checking file {}", file.display());

		let value = Schema::load(&file);
		let filename_maybe = DocFileName::try_from(file);
		if let Ok(_value) = value {
			if let Ok(filename) = filename_maybe {
				(filename.into(), true)
			} else {
				(file.into(), false)
			}
		} else if let Ok(f) = filename_maybe {
			(f.into(), false)
		} else {
			(file.into(), false)
		}
	}

	pub(crate) fn check_files_in_folder(dir: &PathBuf) -> error::Result<HashSet<CheckResult>> {
		log::debug!("Checking all files in folder {}", dir.display());

		let files = DocFile::find(dir, false)?;
		let hs: HashSet<CheckResult> = files.map(|f| Self::check_file(&f)).collect();
		Ok(hs)
	}

	/// Check a list of PRDoc files based on:
	///  - a `file` containing the list of PR numbers
	///  - a base `dir` where to look for those PRDoc files
	pub(crate) fn check_list(file: &PathBuf, dir: &PathBuf) -> error::Result<HashSet<CheckResult>> {
		let extract_numbers = get_numbers_from_file(file)?;

		let numbers: Vec<PRNumber> =
			extract_numbers.iter().filter_map(|(_, _, n)| n.to_owned()).collect();

		Self::check_numbers(numbers, dir)
	}

	/// Return true if all checks were OK, false otherwise.
	pub fn global_result(hs: HashSet<CheckResult>) -> bool {
		for item in hs.iter() {
			if !item.1 {
				return false;
			}
		}

		true
	}

	/// Run the check: considering an input directory and either a file, some numbers, of a list
	/// file, run thru the list and check the validity of the PRDoc files.
	/// We return a Vec instead of a HashSet because a check based on a file may not always lead
	/// to a PR number, making the HashSet made of a bunch of (None, bool).
	pub fn run(
		dir: &PathBuf,
		file: Option<PathBuf>,
		numbers: Option<Vec<PRNumber>>,
		list: Option<PathBuf>,
	) -> crate::error::Result<HashSet<CheckResult>> {
		log::debug!("Checking directory {}", dir.display());

		match (file, numbers, list) {
			(Some(file), None, None) => {
				let file =
					if file.is_relative() { Path::new(&dir).join(&file) } else { file.clone() };
				let mut hs = HashSet::new();
				let _ = hs.insert(Self::check_file(&file));
				Ok(hs)
			},

			(None, Some(numbers), None) => Self::check_numbers(numbers, dir),
			(None, None, Some(list)) => Self::check_list(&list, dir),
			(None, None, None) => Self::check_files_in_folder(dir),

			_ => unreachable!(),
		}
	}
}
