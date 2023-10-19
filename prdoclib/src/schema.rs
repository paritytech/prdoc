//! Schema used by [prdoc](/prdoc) for its validation
//!
//! [prdoc](/prdoc) does not really care about the schema itself and the data is not used or loaded.
//! The schema is stored in the repository and embedded into the cli for convenience. The various
//! commands do check that files comply with the schema but nothing more. That also means that the
//! schema can be adjusted at any time without impact on the code.

use crate::error::PRdocLibError;
use regex::Regex;
use serde_yaml::Value;
use std::{fs::File, path::Path};
use valico::json_schema;

/// Default schema for the validation of data provided by developers
pub const JSON_SCHEMA: &str = include_str!("./prdoc_schema_user.json");

/// Default file extension
pub const EXTENSION: &str = "prdoc";

/// Default location where prdoc are stored
pub const PRDOC_DEFAULT_DIR: &str = "prdoc";

/// The schema embedded in [prdoc](/prdoc).
pub struct Schema {}

impl Schema {
	/// JSON Schema sometimes do contain comments. This function strips them to allow
	/// proper deserialization.
	pub fn get(strip_comments: bool) -> String {
		if !strip_comments {
			JSON_SCHEMA.to_string()
		} else {
			let re = Regex::new(r"(?m)^//(.*)$").unwrap();
			let result = re.replace_all(JSON_SCHEMA, "");
			result.to_string().trim().to_string()
		}
	}

	/// Check the validity of a file by attempting to load it
	pub fn check_file<P: AsRef<Path>>(file: &P) -> bool {
		Self::load(file).is_ok()
	}

	/// Load the content of a file. The name does not matter here.
	pub fn load<P: AsRef<Path>>(file: &P) -> crate::error::Result<Value> {
		let schema_str = Self::get(true);
		let json_schema: serde_json::Value = serde_json::from_str(&schema_str)?;

		let reader = File::open(file)?;
		let mut doc_as_yaml: serde_yaml::Value = serde_yaml::from_reader(reader)?;
		doc_as_yaml.apply_merge()?;

		let doc_as_json: serde_json::Value =
			serde_yaml::from_value(serde_yaml::to_value(&doc_as_yaml)?)?;

		let mut scope = json_schema::Scope::new();
		let schema = scope.compile_and_return(json_schema, false)?;

		let validation = schema.validate(&doc_as_json);
		let validation_result = validation.is_valid();
		let validation_result_strict = validation.is_strictly_valid();

		if !(validation_result && validation_result_strict) {
			log::warn!("validation_result: {validation_result}");
			log::warn!("validation_result_strict: {validation_result_strict}");
			log::warn!("errors: {:#?}", validation.errors);
			log::warn!("missing: {:#?}", validation.missing);
			return Err(PRdocLibError::ValidationErrors(validation))
		}

		Ok(doc_as_yaml)
	}
}

#[cfg(test)]
mod test_schema_validation {
	use super::*;
	use std::path::PathBuf;

	#[test]
	fn test_load_valid_1234() {
		let file = PathBuf::from("../tests/data/some/pr_1234_some_test_minimal.prdoc");
		assert!(Schema::load(&file).is_ok());
	}

	#[test]
	fn test_check_valid_1234() {
		let file = PathBuf::from("../tests/data/some/pr_1234_some_test_minimal.prdoc");
		assert!(Schema::check_file(&file));
	}
}
