use crate::{error::Result, utils::get_project_root};
use serde::Deserialize;
use std::{fs, path::PathBuf, str::FromStr};
use toml::Table;

const NAMES: &'static [&'static str] = &["prdoc.toml", ".prdoc.toml"];

#[derive(Debug, Deserialize)]
pub struct PRDocConfig {
	schema: PathBuf,
}

pub struct Config;

impl Config {
	/// Try finding the config in various locations:
	/// - $PROJECT_ROOT/prdoc.toml
	/// - $PROJECT_ROOT/.prdoc.toml
	/// - $PRDOC_CONFIG
	pub fn get_config_file(config: Option<PathBuf>) -> Result<PathBuf> {
		if PathBuf::from("prdoc.toml").exists() {
            log::debug!("Found config in prdoc.toml");
			return Ok(PathBuf::from("prdoc.toml"))
		}

		if PathBuf::from(".prdoc.toml").exists() {
            log::debug!("Found config in .prdoc.toml");
			return Ok(PathBuf::from(".prdoc.toml"))
		}

        // TODO: not needed
		// if let Ok(config) = std::env::var("PRDOC_CONFIG") {
        //     if PathBuf::from(&config).exists() {
        //         log::debug!("Found config in {config}");
		// 		return Ok(PathBuf::from(config))
		// 	}
		// }

        log::error!("Config not found");
		Err(crate::error::PRdocLibError::MissingConfig)
	}

	// fn get_config_from_cargo_toml() -> Result<PRDocConfig> {
	//     let root = get_project_root()?;
	//     let cargo_toml = root.join("Cargo.toml");
	//     match fs::read_to_string(cargo_toml)?.parse() {
	//         Ok(c) => Ok(c),
	//         Err(_) => todo!(),
	//     }
	//     // let toml = "foo = 'bar'".parse::<Table>().unwrap();

	//     // Err(crate::error::PRdocLibError::InvalidConfig(cargo_toml))
	// }
}
