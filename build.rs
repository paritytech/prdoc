use std::{borrow::Cow, process::Command};

fn main() {
	generate_cargo_keys();
}

/// Generate the cargo keys
pub fn generate_cargo_keys() {
	generate_cargo_key_git();
	generate_cargo_key_build_date();
}

pub fn generate_cargo_key_build_date() {
	let build_date = match Command::new("date").args(["-u", "+%FT%TZ"]).output() {
		Ok(o) if o.status.success() => {
			let sha = String::from_utf8_lossy(&o.stdout).trim().to_owned();
			Cow::from(sha)
		},
		Ok(o) => {
			let status = o.status;
			println!("cargo:warning=Failed fetching the date timestamp: {status}");
			Cow::from("unknown")
		},
		Err(err) => {
			println!("cargo:warning=Failed fetching the datge: {err}");
			Cow::from("unknown")
		},
	};

	println!("cargo:rustc-env=PRDOC_CLI_BUILD_DATE={build_date}");
}

pub fn generate_cargo_key_git() {
	let commit = if let Ok(hash) = std::env::var("PRDOC_CLI_GIT_COMMIT_HASH") {
		Cow::from(hash.trim().to_owned())
	} else {
		match Command::new("git").args(["rev-parse", "--short=11", "HEAD"]).output() {
			Ok(o) if o.status.success() => {
				let tmsp = String::from_utf8_lossy(&o.stdout).trim().to_owned();
				Cow::from(tmsp)
			},
			Ok(o) => {
				let status = o.status;
				println!("cargo:warning=Git command failed with status: {status}");
				Cow::from("unknown")
			},
			Err(err) => {
				println!("cargo:warning=Failed to execute git command: {err}");
				Cow::from("unknown")
			},
		}
	};

	println!("cargo:rustc-env=PRDOC_CLI_GIT_COMMIT_HASH={commit}");
}
