//! Implementation of the load command. This command loads a PRDoc file and outputs its content as
//! YAML or JSON. Load can also work on several files and aggregates its output as array in that
//! case.

use crate::{
	common::PRNumber,
	config::PRDocConfig,
	doc_filename::DocFileName,
	docfile::DocFile,
	docfile_wrapper::DocFileWrapper,
	error::{self, Result},
	schema::Schema,
	utils::{get_numbers_from_file, get_project_root},
};
use std::{
	collections::HashSet,
	path::{Path, PathBuf},
};

/// Wrapper for the load command
pub struct LoadCmd {
	pub(crate) schema: Schema,
}

/// Type alias for the load command result
pub type LoadResult = (bool, HashSet<DocFileWrapper>);

impl LoadCmd {
	/// Create a new instance of the load command
	pub fn new(schema: Schema) -> Self {
		Self { schema }
	}

	/// Load PRDoc from one or more numbers
	pub(crate) fn load_numbers(
		&self,
		numbers: Vec<PRNumber>,
		dir: &PathBuf,
	) -> error::Result<LoadResult> {
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

						let yaml = self.schema.load(&file);

						if let Ok(value) = yaml {
							Some(DocFileWrapper::new(file, filename, Some(value)))
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
	pub fn load_file(&self, file: &PathBuf) -> Result<DocFileWrapper> {
		let filename = DocFileName::try_from(file)?;
		let value = self.schema.load(&file).ok();
		let wrapper = DocFileWrapper::new(file.clone(), filename, value);
		Ok(wrapper)
	}

	pub(crate) fn load_list(&self, file: &PathBuf, dir: &PathBuf) -> Result<LoadResult> {
		let extract_numbers = get_numbers_from_file(file)?;
		let numbers: Vec<PRNumber> =
			extract_numbers.iter().filter_map(|(_, _, n)| n.to_owned()).collect();

		let mut global_result = extract_numbers.iter().map(|(_, status, _)| status).all(|&x| x);

		let (r, wrapper) = self.load_numbers(numbers, dir).unwrap();
		global_result &= r;
		Ok((global_result, wrapper))
	}

	pub(crate) fn load_from_folder(&self, dir: &PathBuf) -> Result<LoadResult> {
		let res = DocFile::find(self.schema.clone(), dir, false);
		let mut global_result = res.is_ok();

		let wrapper = res
			.unwrap()
			.filter_map(|file| {
				let filename_maybe = DocFileName::try_from(&file);

				if let Ok(filename) = filename_maybe {
					let yaml = self.schema.load(&file);
					if let Ok(value) = yaml {
						let wrapper = DocFileWrapper::new(file.clone(), filename, Some(value));

						global_result &= true;
						log::debug!("OK  {}", file.display());
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

	/// Run of the load command
	pub fn run(
		config: &PRDocConfig,
		schema: Option<PathBuf>,
		dir: &PathBuf,
		file: Option<PathBuf>,
		numbers: Option<Vec<PRNumber>>,
		list: Option<PathBuf>,
	) -> Result<LoadResult> {
		log::debug!("Loading from directory {}", dir.display());

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
		let load_cmd = LoadCmd::new(schema);

		Ok(match (file, numbers, list) {
			(Some(f), None, None) => {
				let file_abs = if f.is_relative() { Path::new(&dir).join(&f) } else { f.clone() };
				let mut wrapper = HashSet::new();
				wrapper.insert(load_cmd.load_file(&file_abs)?);

				(true, wrapper)
			},

			(None, Some(numbers), None) => {
				log::debug!("Loading numbers {:?}", numbers);
				let (global_result, wrapper) = load_cmd.load_numbers(numbers, dir)?;
				(global_result, wrapper)
			},

			(None, None, Some(list)) => {
				log::debug!("Loading list from {:?}", list);
				let (global_result, wrapper) = load_cmd.load_list(&list, dir)?;
				(global_result, wrapper)
			},

			(None, None, None) => {
				log::debug!("Loading all files in folder {}", dir.display());
				let (global_result, wrapper) = load_cmd.load_from_folder(dir)?;
				(global_result, wrapper)
			},

			_ => unreachable!(),
		})
	}
}
