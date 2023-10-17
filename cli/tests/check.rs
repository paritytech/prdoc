mod common;

#[cfg(test)]
mod cli_tests {

	#[cfg(test)]
	mod check {
		use crate::common::*;
		use prdoclib::config;

		#[test]
		fn it_check_passes_without_args() {
			let mut cmd = prdoc_bin();
			cmd.env_clear().env(config::env::PRDOC_FOLDER, "../tests/data/all");

			let assert = cmd.arg("check").assert();
			assert.success().code(exitcode::OK);
		}

		#[test]
		fn it_check_with_number_when_valid() {
			let mut cmd = prdoc_bin();

			let assert = cmd.args(["check", "-d", "../tests/data/some", "-n", "1234"]).assert();
			assert.success().code(exitcode::OK);
		}

		#[test]
		fn it_check_with_number_when_invalid() {
			let mut cmd = prdoc_bin();

			let assert = cmd.args(["check", "-d", "../tests/data/some", "-n", "1"]).assert();
			assert.failure().code(exitcode::DATAERR);
		}

		#[test]
		fn it_check_with_file_when_valid() {
			let mut cmd = prdoc_bin();
			let assert = cmd.args(["check", "-f", "pr_1234_some_test_minimal.prdoc"]).assert();
			assert.success().code(exitcode::OK);
		}

		#[test]
		fn it_check_with_file_and_dir_when_valid() {
			let mut cmd = prdoc_bin();

			let assert = cmd.args(["check", "-f", "pr_1234_some_test_minimal.prdoc"]).assert();
			assert.success().code(exitcode::OK);
		}

		#[test]
		fn it_check_with_list_all() {
			let mut cmd = prdoc_bin();

			let assert = cmd.args(["check", "--list", "../tests/data/lists/simple/all_good.txt"]).assert();
			assert.success().code(exitcode::OK);
		}

		#[test]
		fn it_check_with_list_some() {
			let mut cmd = prdoc_bin();

			let assert = cmd.args(["check", "--list", "../tests/data/lists/simple/some_good.txt"]).assert();
			assert.failure().code(exitcode::DATAERR);
		}
	}
}
