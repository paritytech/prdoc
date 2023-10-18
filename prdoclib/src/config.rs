use crate::{error::Result, utils::get_project_root};
use serde::Deserialize;
use std::{fs, path::PathBuf};
// use toml::Table;

const CONFIG_NAMES: &[&str] = &["prdoc.toml", ".prdoc.toml"];

pub mod env {
	pub const PRDOC_CONFIG: &str = "PRDOC_CONFIG";
	pub const PRDOC_FOLDER: &str = "PRDOC_FOLDER";
}

#[derive(Debug, Deserialize)]
pub struct PRDocConfig {
	pub version: u16,
	pub schema: PathBuf,

	/// Used for load, scan, check
	pub prdoc_folders: Vec<PathBuf>,

	/// Used by the generate command
	pub out_dir: PathBuf,
}

pub struct Config;

impl Config {
	/// Try finding the PRDOc config in various locations:
	/// - $PROJECT_ROOT/prdoc.toml
	/// - $PROJECT_ROOT/.prdoc.toml
	/// - $PRDOC_CONFIG
	pub fn get_config_file(config_file: Option<PathBuf>) -> Result<PathBuf> {
		let root = get_project_root().unwrap();

		for name in CONFIG_NAMES {
			if root.join(name).exists() {
				log::debug!("Found config in {name}");
				return Ok(PathBuf::from(name))
			}
		}

		// if root.join(".prdoc.toml").exists() {
		// 	log::debug!("Found config in .prdoc.toml");
		// 	return Ok(PathBuf::from(".prdoc.toml"));
		// }

		if let Some(config) = config_file {
			if PathBuf::from(&config).exists() {
				log::debug!("Found config in {config:?}");
				return Ok(config)
			}
		}

		log::warn!("Config not found");
		Err(crate::error::PRdocLibError::MissingConfig)
	}

	pub fn get_default_config() -> PRDocConfig {
		PRDocConfig::default()
	}

	pub fn load(config_opts: Option<PathBuf>) -> Result<PRDocConfig> {
		let config_file = Self::get_config_file(config_opts)?;
		log::debug!("Loading {config_file:?}");
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
			version: 1,
			schema: "prdoc_schema__user.json".into(),
			prdoc_folders: vec!["prdoc".into()],
			out_dir: "prdoc".into(),
		}
	}
}
