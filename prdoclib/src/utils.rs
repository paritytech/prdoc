use std::{
	env,
	fs::read_dir,
	io::{self, ErrorKind},
	path::PathBuf,
};

use crate::{common::PRNumber, error};

pub type ParseResult = (String, bool, Option<PRNumber>);

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

/// Return the path of the folder where PRDoc are stored
pub fn get_pr_doc_folder() -> io::Result<PathBuf> {
	// TODO: Use config as well
	let root = get_project_root()?;
	let subdir = match env::var("PRDOC_DIR") {
		Ok(v) => v,
		Err(_) => "prdoc".to_string(),
	};

	let dir = root.join(subdir);
	Ok(dir)
}
