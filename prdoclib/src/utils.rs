use std::{
	env,
	ffi::OsString,
	fs::read_dir,
	io::{self, ErrorKind},
	path::PathBuf,
};

pub fn get_project_root() -> io::Result<PathBuf> {
	let path = env::current_dir()?;
	let mut path_ancestors = path.as_path().ancestors();

	while let Some(p) = path_ancestors.next() {
		let has_cargo = read_dir(p)?
			.into_iter()
			.any(|p| p.unwrap().file_name() == OsString::from("Cargo.lock"));
		if has_cargo {
			return Ok(PathBuf::from(p));
		}
	}
	Err(io::Error::new(ErrorKind::NotFound, "Ran out of places to find Cargo.toml"))
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
