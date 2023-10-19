//! Set of utils
use crate::{common::PRNumber, config::PRDocConfig, error};
use std::{
	env,
	fs::read_dir,
	io::{self, ErrorKind},
	path::PathBuf,
};

/// Type alias for the result of parsing a file
pub type ParseResult = (String, bool, Option<PRNumber>);

/// Get the project root (relative to closest Cargo.lock file)
/// ```rust
/// match project_root::get_project_root() {
///     Ok(p) => println!("Current project root is {:?}", p),
///     Err(e) => println!("Error obtaining project root {:?}", e)
/// };
/// ```
pub fn get_project_root() -> io::Result<PathBuf> {
	let path = env::current_dir()?;
	let path_ancestors = path.as_path().ancestors();

	for p in path_ancestors {
		let has_cargo = read_dir(p)?.any(|p| p.unwrap().file_name() == "Cargo.lock");
		if has_cargo {
			return Ok(PathBuf::from(p))
		}
	}
	Err(io::Error::new(ErrorKind::NotFound, "Ran out of places to find Cargo.toml"))
}

pub(crate) fn get_numbers_from_file(file: &PathBuf) -> error::Result<Vec<ParseResult>> {
	Ok(std::fs::read_to_string(file)?
		.lines()
		.map(|line| {
			let num = line.parse::<PRNumber>();
			let valid = num.is_ok();
			let number = num.ok();
			(line.to_string(), valid, number)
		})
		.collect())
}

/// Return the path of the folder where PRDoc are stored
pub fn get_pr_doc_folder(output_dir: Option<PathBuf>, config: &PRDocConfig) -> PathBuf {
	if let Some(path) = output_dir {
		return path
	}

	config.output_dir.clone()
}

/// Get the template path from the config
pub fn get_template_path(config: &PRDocConfig) -> PathBuf {
	config.template.clone()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_should_find_our_project_root() {
		let project_root = get_project_root().expect("There is no project root");
		let toml_path = project_root.to_str().unwrap().to_owned() + "/Cargo.toml";
		assert!(!toml_path.is_empty());
	}
}
