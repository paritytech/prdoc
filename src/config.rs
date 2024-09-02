//! PRDoc config

use crate::{error::Result, utils::get_project_root};
use serde::Deserialize;
use std::{fs, path::PathBuf};

const CONFIG_NAMES: &[&str] = &["prdoc.toml", ".prdoc.toml"];

/// Environment variables used by PRDoc
pub mod env {
	/// If the config is not located at the root of the project or does not have standard name, it
	/// can still be provided via this env variable
	pub const PRDOC_CONFIG: &str = "PRDOC_CONFIG";

	/// Not fully supported yet
	//TODO: Add proper support
	pub const PRDOC_FOLDERS: &str = "PRDOC_FOLDERS";
}

/// PRDoc config
#[derive(Debug, Deserialize)]
pub struct PRDocConfig {
	// /// Config version
	// pub(crate) version: u16,
	/// Path of the schema
	pub(crate) schema: PathBuf,

	/// Used for load, scan, check
	pub prdoc_folders: Vec<PathBuf>,

	/// Used by the generate command
	pub(crate) output_dir: PathBuf,

	/// Path of the file to use as template
	pub(crate) template: PathBuf,
}

/// Wrapper struct for the `PRDocConfig`
pub struct Config;

impl Config {
	/// Try finding the PRDOc config in various locations:
	/// - $PROJECT_ROOT/prdoc.toml
	/// - $PROJECT_ROOT/.prdoc.toml
	/// - $PRDOC_CONFIG
	pub fn get_config_file(config_file: Option<PathBuf>) -> Result<PathBuf> {
		let root = get_project_root().expect("prdoc should run in a repo");

		if let Some(config) = config_file {
			if PathBuf::from(&config).exists() {
				log::debug!("Found config in {config:?}");
				return Ok(config);
			}
		}

		for name in CONFIG_NAMES {
			let candidate = root.join(name);
			if candidate.exists() {
				log::debug!("Found config in {}", candidate.display());
				return Ok(candidate);
			}
		}

		log::warn!("Config not found");
		Err(crate::error::PRdocLibError::MissingConfig)
	}

	/// Return a default config. This is used when no config was found or the config file is invalid
	pub fn get_default_config() -> PRDocConfig {
		PRDocConfig::default()
	}

	/// Load the config from the config file
	pub fn load(config_opts: Option<PathBuf>) -> Result<PRDocConfig> {
		let config_file = Self::get_config_file(config_opts)?;
		log::debug!("Loading config from {config_file:?}");
		let str = match fs::read_to_string(config_file.clone()) {
			Ok(s) => s,
			Err(_) => Err(crate::error::PRdocLibError::InvalidConfig(config_file.clone()))?,
		};

		match toml::from_str(str.as_str()) {
			Ok(c) => Ok(c),
			Err(_e) => Err(crate::error::PRdocLibError::InvalidConfig(config_file))?,
		}
	}
}

impl Default for PRDocConfig {
	fn default() -> Self {
		Self {
			// version: 1,
			schema: "prdoc/schema_user.json".into(),
			prdoc_folders: vec!["prdoc".into()],
			output_dir: "prdoc".into(),
			template: "template.prdoc".into(),
		}
	}
}

impl PRDocConfig {
	/// Return the path of the schema
	pub fn schema_path(&self) -> PathBuf {
		self.schema.clone()
	}
}
