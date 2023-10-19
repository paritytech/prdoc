use assert_cmd::Command;
use prdoclib::config;

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn prdoc_bin() -> Command {
	let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
	cmd.env(config::env::PRDOC_FOLDERS, "../tests/data/all");
	cmd
}
