#[cfg(test)]
mod cli_tests {
	#[cfg(test)]
	mod schema {
		use assert_cmd::Command;

		#[test]
		fn it_provides_a_schema() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
			let assert = cmd.arg("schema").assert();
			assert.success().code(exitcode::OK);
		}
	}
}
