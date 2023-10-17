mod common;

#[cfg(test)]
mod cli_tests {
	#[cfg(test)]
	mod scan {
		use crate::common::prdoc_bin;

		#[test]
		fn it_provides_a_scan() {
			let mut cmd = prdoc_bin();
			let assert = cmd.args(["-d", "../tests/data/some"]).args(["scan"]).assert();
			assert.success().code(exitcode::OK);
		}
	}
}
