#[cfg(test)]
mod cli_tests {
	#[cfg(test)]
	mod check {
		use assert_cmd::Command;

		#[test]
		fn it_loads_one_file() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
			let assert = cmd.arg("load").args(["--file", "pr_1237.prdoc"]).assert();
			assert.success().code(exitcode::OK);
		}

		#[test]
		fn it_loads_one_by_number() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
			let assert = cmd.arg("load").args(["-n", "1237"]).assert();
			assert.success().code(exitcode::OK);
		}

		#[test]
		fn it_loads_some_by_number() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
			let assert = cmd.arg("load").args(["-n", "1234", "-n", "1237"]).assert();
			assert.success().code(exitcode::OK);
		}

		#[test]
		fn it_loads_all_by_folder() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
			let assert = cmd.arg("load").args(["-d", "../tests/data/all"]).assert();
			assert.success().code(exitcode::OK);
		}

		#[test]
		fn it_loads_some_by_folder() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
			let assert = cmd.arg("load").args(["-d", "../tests/data/some"]).assert();
			assert.failure().code(65);
		}

		#[test]
		fn it_loads_all_by_list() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
			let assert = cmd
				.arg("load")
				.args(["-d", "../tests/data/all", "--list", "../tests/data/lists/simple/all_good.txt"])
				.assert();
			assert.success().code(exitcode::OK);
		}

		#[test]
		fn it_loads_some_by_list() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
			let assert = cmd
				.arg("load")
				.args(["-d", "../tests/data/some", "--list", "../tests/data/lists/simple/some_good.txt"])
				.assert();
			assert.failure().code(65);
		}
	}
}
