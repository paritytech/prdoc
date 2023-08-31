use serde_json::json;

pub struct VersionCmd;

impl VersionCmd {
	pub fn run(name: &str, version: &str, json: bool) {
		let commit_hash = std::env::var("PRDOC_CLI_GIT_COMMIT_HASH");
		let build_date = std::env::var("PRDOC_CLI_BUILD_DATE");

		if !json {
			let commit_hash_str =
				if let Ok(s) = commit_hash { format!("-{s}") } else { String::from("") };
			let build_date_str =
				if let Ok(s) = build_date { format!(" built {s}") } else { String::from("") };
			println!("{name} v{version}{commit_hash_str}{build_date_str}");
		} else {
			let version_data = json!({
				"name": name,
				"version": version,
				"commit": commit_hash.unwrap_or_default(),
				"build_date": build_date.unwrap_or_default(),
			});
			let s =
				serde_json::to_string_pretty(&version_data).expect("serde_json ran into issues");
			println!("{s}");
		}
	}
}
