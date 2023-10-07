#[cfg(test)]
mod cli_tests {
	#[cfg(test)]
	mod generate {
		use assert_cmd::Command;

		#[test]
		fn it_generate_fails_without_number() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
			let assert = cmd.arg("generate").assert();
			assert.failure().code(2);
		}

		#[test]
		fn it_generate_with_number() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
			let assert = cmd.args(["generate", "--dry-run", "42"]).assert();
			assert.success().code(exitcode::OK);
		}
	}
}
