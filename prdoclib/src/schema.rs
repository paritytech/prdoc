use regex::Regex;
use serde_yaml::Value;
use std::{fs::File, path::Path};
use valico::json_schema;

use crate::error::PRdocLibError;

pub const JSON_SCHEMA: &str = include_str!("../../schema.json");
pub const EXTENSION: &str = "prdoc";
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

	pub fn check<P: AsRef<Path>>(file: P) -> bool {
		Self::load(file).is_ok()
	}

	pub fn load<P: AsRef<Path>>(file: P) -> crate::error::Result<Value> {
		let schema_str = Self::get(true);
		let json_schema: serde_json::Value = serde_json::from_str(&schema_str).unwrap();

		let reader = File::open(file).unwrap();
		let doc_as_yaml: serde_yaml::Value = serde_yaml::from_reader(reader).unwrap();

		let doc_as_json: serde_json::Value =
			serde_yaml::from_value(serde_yaml::to_value(&doc_as_yaml).unwrap()).unwrap();
		// let doc_as_json = serde_json::to_string(&yaml).unwrap();

		let mut scope = json_schema::Scope::new();
		let schema = scope.compile_and_return(json_schema, false).unwrap();

		let validation = schema.validate(&doc_as_json);
		let validation_result = validation.is_valid();
		let validation_result_strict = validation.is_strictly_valid();

		if !(validation_result && validation_result_strict) {
			println!("errors: {:#?}", validation.errors);
			println!("missing: {:#?}", validation.missing);
			return Err(PRdocLibError::ValidationErrors(validation));
		}

		Ok(doc_as_yaml)
	}
}
