use std::{fs::File, path::Path};
use regex::Regex;
use serde_json::Value;
use valico::json_schema;

pub const JSON_SCHEMA: &'static str = include_str!("../../schema.json");
pub const EXTENSION: &'static str = "prdoc";
pub struct Schema {}

impl Schema {
	pub fn get(strip_comments: bool) -> String {
		if !strip_comments {
			JSON_SCHEMA.to_string()
		} else {
			let re = Regex::new(r"(?m)^//(.*)$").unwrap();
			let result = re.replace_all(JSON_SCHEMA, "");
			result.to_string().trim().to_string()
		}
	}

	// TODO: Return a Result with the error/missing if ERR
	pub fn check<P: AsRef<Path>>(file: P) -> bool {
		let schema_str = Self::get(true);
		let json_schema: Value = serde_json::from_str(&schema_str).unwrap();
		let yaml: Value = serde_yaml::from_reader(File::open(file).unwrap()).unwrap();

		let mut scope = json_schema::Scope::new();
		let schema = scope.compile_and_return(json_schema, false).unwrap();

		let validation = schema.validate(&yaml);
		let validation_result = validation.is_valid();
		let validation_result_strict = validation.is_strictly_valid();

		// println!("{title:<15}: {validation_result}", title = "valid");
		// println!("{title:<15}: {validation_result_strict}\n", title = "strictly valid");

		// todo: add to the returned Err
		// if !(validation_result && validation_result_strict) {
		// 	println!("errors: {:#?}", validation.errors);
		// 	println!("missing: {:#?}", validation.missing);
		// }

		validation_result && validation_result_strict
	}
}
