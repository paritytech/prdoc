//! Implementation of the check command. This command attempts to load a PRDoc file and checks
//! whether it adheres to the schema or not.

use crate::{
	common::PRNumber,
	config::PRDocConfig,
	doc_filename::DocFileName,
	docfile::DocFile,
	error::{self},
	prdoc_source::PRDocSource,
	schema::Schema,
	utils::{get_numbers_from_file, get_project_root},
};
use std::{
	collections::HashSet,
	path::{Path, PathBuf},
};

/// Implementation of the main [check](/prdoc::opts::CheckOpts) command of the cli.
pub struct CheckCmd {
	pub(crate) schema: Schema,
}

/// PRDoc are checked via a PR number or a file.
/// - When passing a file path, it may not result in a `PRNumber`.
/// - When passing a PRNumber, it also may not result in a file
pub type CheckResult = (PRDocSource, bool);

impl CheckCmd {
	/// Create a new instance of the check command
	pub fn new(schema: Schema) -> Self {
		Self { schema }
	}

	pub(crate) fn check_numbers(
		&self,
		numbers: Vec<PRNumber>,
		dir: &PathBuf,
	) -> error::Result<HashSet<CheckResult>> {
		log::debug!("Checking PRs: {:?}", numbers);

		let res = numbers
			.iter()
			.map(|&number| {
				log::debug!("Checking PR #{}", number);

				let file_maybe = DocFileName::find(number, None, dir);

				match file_maybe {
					Ok(file) => {
						log::debug!("Attempting to load file: {}", file.display());
						let yaml = self.schema.load(&file);

						match yaml {
							Ok(_value) => {
								log::debug!("Loading was OK");
								(number.into(), true)
							},
							Err(e) => {
								log::error!("Loading the schema failed:");
								log::error!("{e:?}");
								(number.into(), false)
							},
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
	pub(crate) fn _check_number(
		&self,
		number: PRNumber,
		dir: &PathBuf,
	) -> error::Result<CheckResult> {
		let file = DocFileName::find(number, None, dir)?;
		Ok((file.clone().into(), self.check_file(&file).1))
	}

	/// Check a specific file given its full path.
	/// All the other check_xxx functions are based on this one.
	pub(crate) fn check_file(&self, file: &PathBuf) -> CheckResult {
		log::debug!("Checking file {}", file.display());

		let value = self.schema.load(&file);
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

	/// Check all files in a given folder. The dot files (ie filenames starting with a dot) are
	/// ignored This functions allows checking all files or only the valid ones thanks to the
	/// `valid_only` argument.
	pub(crate) fn check_files_in_folder(
		self,
		dir: &PathBuf,
		valid_only: bool,
	) -> error::Result<HashSet<CheckResult>> {
		log::debug!("Checking all files in folder {}", dir.display());

		let schema = self.schema.clone();
		let files = DocFile::find(schema, dir, valid_only)?
			.filter(|f| !f.file_name().unwrap_or_default().to_string_lossy().starts_with('.'));
		let hs: HashSet<CheckResult> = files.map(|f| self.check_file(&f)).collect();
		Ok(hs)
	}

	/// Check a list of PRDoc files based on:
	///  - a `file` containing the list of PR numbers
	///  - a base `dir` where to look for those PRDoc files
	pub(crate) fn check_list(
		&self,
		file: &PathBuf,
		dir: &PathBuf,
	) -> error::Result<HashSet<CheckResult>> {
		let extract_numbers = get_numbers_from_file(file)?;

		let numbers: Vec<PRNumber> =
			extract_numbers.iter().filter_map(|(_, _, n)| n.to_owned()).collect();

		self.check_numbers(numbers, dir)
	}

	/// Return true if all checks were OK, false otherwise.
	pub fn global_result(hs: HashSet<CheckResult>) -> bool {
		for item in hs.iter() {
			if !item.1 {
				return false
			}
		}

		true
	}

	/// Run the check: considering an input directory and either a file, some numbers, of a list
	/// file, run thru the list and check the validity of the PRDoc files.
	/// We return a Vec instead of a HashSet because a check based on a file may not always lead
	/// to a PR number, making the HashSet made of a bunch of (None, bool).
	pub fn run(
		config: &PRDocConfig,
		schema: Option<PathBuf>,
		dir: &PathBuf,
		file: Option<PathBuf>,
		numbers: Option<Vec<PRNumber>>,
		list: Option<PathBuf>,
	) -> crate::error::Result<HashSet<CheckResult>> {
		log::info!("Checking directory {}", dir.display());
		log::debug!("From dir: {}", dir.canonicalize().unwrap().display());

		let repo_root = get_project_root()?;
		log::debug!("From repo root: {}", repo_root.canonicalize().unwrap().display());

		let schema_path = if let Some(schema_path) = schema {
			schema_path
		} else if config.schema_path().is_absolute() {
			config.schema_path()
		} else {
			repo_root.join(config.schema_path())
		};

		log::info!("Using schema: {}", schema_path.canonicalize().unwrap().display());
		let schema = Schema::new(schema_path);

		let check_cmd = CheckCmd::new(schema);

		match (file, numbers, list) {
			(Some(file), None, None) => {
				let file =
					if file.is_relative() { Path::new(&dir).join(&file) } else { file.clone() };

				let mut hs = HashSet::new();
				let _ = hs.insert(check_cmd.check_file(&file));
				Ok(hs)
			},

			(None, Some(numbers), None) => check_cmd.check_numbers(numbers, dir),
			(None, None, Some(list)) => check_cmd.check_list(&list, dir),
			(None, None, None) => check_cmd.check_files_in_folder(dir, false),

			_ => unreachable!(),
		}
	}
}
