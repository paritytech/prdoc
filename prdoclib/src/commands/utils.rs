use std::path::PathBuf;

use crate::{common::PRNumber, error};

pub type ParseResult = (String, bool, Option<PRNumber>);

pub(crate) fn get_numbers_from_file(file: &PathBuf) -> error::Result<Vec<ParseResult>> {
	Ok(std::fs::read_to_string(file)?
		.lines()
		.map(|line| {
			let num = line.parse::<PRNumber>();
			let valid = num.is_ok();
			let number = num.ok();
			(line.to_string(), valid, number)
		})
		.collect())
}
