use std::{path::PathBuf, fs, str::FromStr};
use serde::Deserialize;
use toml::Table;
use crate::{error::Result, utils::get_project_root};

const NAMES: &'static [&'static str]  = &[ "prdoc.toml", ".prdoc.toml" ];

#[derive(Debug, Deserialize, FromStr)]
pub struct PRDocConfig {
    schema: PathBuf,
}


pub struct Config;

impl Config {
    /// Try finding the config in various locations
    pub fn get_config_file() -> Result<PathBuf> {


        todo!()
    }

    fn get_config_from_cargo_toml() -> Result<PRDocConfig> {
        let root = get_project_root()?;
        let cargo_toml = root.join("Cargo.toml");
        match fs::read_to_string(cargo_toml)?.parse() {
            Ok(c) => Ok(c),
            Err(_) => todo!(),
        }
        // let toml = "foo = 'bar'".parse::<Table>().unwrap();

        // Err(crate::error::PRdocLibError::InvalidConfig(cargo_toml))
    }
}
