mod common;

#[cfg(test)]
mod cli_tests {
	#[cfg(test)]
	mod generate {
		use crate::common::prdoc_bin;
		use std::fs;

		#[test]
		fn it_generate_fails_without_number() {
			let mut cmd = prdoc_bin();
			let assert = cmd.arg("generate").assert();
			assert.failure().code(2);
		}

		#[test]
		fn it_generate_with_number() {
			let mut cmd = prdoc_bin();
			let assert = cmd.args(["generate", "--dry-run", "42"]).assert();
			assert.success().code(exitcode::OK);
		}

		#[test]
		fn it_does_not_overwrite() {
			// Ensure we start without a file so the first generate always work
			let _ = fs::remove_file("/tmp/prdoc/pr_9999.prdoc");

			let mut cmd = prdoc_bin();
			let assert = cmd.args(["generate", "--output-dir", "/tmp/prdoc", "9999"]).assert();
			assert.success().code(exitcode::OK);

			let mut cmd = prdoc_bin();
			let assert = cmd.args(["generate", "--output-dir", "/tmp/prdoc", "9999"]).assert();
			assert.failure().code(exitcode::IOERR);
		}

		#[test]
		fn it_should_not_fail() {
			let _ = fs::remove_file("/tmp/pr_9999.prdoc");

			let mut cmd = prdoc_bin();
			let assert = cmd.args(["generate", "-d", "/tmp", "9999"]).assert();
			assert.success().code(exitcode::OK);
		}
	}
}
