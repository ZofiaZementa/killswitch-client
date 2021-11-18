use anyhow::{Context, Result};
use serde::Deserialize;
use serde_yaml;
use std::{collections::BTreeMap, fs::File};

#[derive(Deserialize, Debug)]
pub struct Check {
    pub cmd: String,
    #[serde(default = "kill_on_success_default")]
    pub kill_on_success: bool,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    // TODO implement default
    pub kill_cmd: String,
    #[serde(default)]
    pub checks: BTreeMap<String, Check>,
}

fn kill_on_success_default() -> bool {
    false
}

pub fn get_config(path: &str) -> Result<Config> {
    let f = File::open(path).with_context(|| format!("Couldn't open config file {}", path))?;
    Ok(serde_yaml::from_reader(f)?)
}
