#[cfg(test)]
mod cli_tests {
	#[cfg(test)]
	mod scan {
		use assert_cmd::Command;

		#[test]
		fn it_provides_a_scan() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
			let assert = cmd.args(["scan", "../tests/data"]).assert();
			assert.success().code(exitcode::OK);
		}
	}
}
