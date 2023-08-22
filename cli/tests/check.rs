#[cfg(test)]
mod cli_tests {
	#[cfg(test)]
	mod check {
		use assert_cmd::Command;

		#[test]
		fn it_check_fails_without_args() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
			let assert = cmd.arg("check").assert();
			assert.failure().code(exitcode::DATAERR);
		}

		#[test]
		fn it_check_with_number_when_valid() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
			let assert = cmd.args(["check", "-d", "../tests/data", "-n", "1234"]).assert();
			assert.success().code(exitcode::OK);
		}

		#[test]
		fn it_check_with_number_when_invalid() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
			let assert = cmd.args(["check", "-d", "../tests/data", "-n", "1"]).assert();
			assert.failure().code(exitcode::DATAERR);
		}

		#[test]
		fn it_check_with_file_when_valid() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
			let assert = cmd.args(["check", "-f", "../tests/data/pr_1234_some_test_minimal.prdoc"]).assert();
			assert.success().code(exitcode::OK);
		}

		#[test]
		fn it_check_with_file_and_dir_when_valid() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
			let assert = cmd.args(["check", "-d", "../tests/data", "-f", "pr_1234_some_test_minimal.prdoc"]).assert();
			assert.success().code(exitcode::OK);
		}
	}
}
