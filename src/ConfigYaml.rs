use serde::{Deserialize, Serialize};
use yaml_serde::{Value as yamlValue};

#[derive(serde::Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct AdminOptions {
    pub name    :   String,
    pub password:   String,
    pub hash    :   String,
}
#[derive(serde::Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct SVG {
    elements        : yamlValue,
    optimization    : yamlValue,
}

#[derive(serde::Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Paths {
    pub saves   : String,
    pub dir     : yamlValue,
    pub files   : yamlValue,
}

#[derive(serde::Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ServerOptions {
    pub ms_input        : u64,
    pub voting_period   : u64,
    pub poll_actions    : usize,
    pub polling         : bool,
    pub paused          : bool,
}

#[derive(serde::Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct GamemodeDefaults {
    pub democracy_defaults  : ServerOptions,
    pub anarchy_defaults    : ServerOptions,
    pub pause_defaults      : ServerOptions,
    pub initial_state       : usize,
    // 0 is democracy, 1 is anarchy, 2 is paused
}

#[derive(serde::Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub administration  : AdminOptions,
    pub svg             : SVG,
    pub paths           : Paths,
    #[serde(rename = "gamemode-defaults")]
    pub gamemodeDefaults: GamemodeDefaults,
}

const CONFIG_PATH: &str = "./config.yaml";

pub fn readYaml() -> Result<Config, Box<dyn std::error::Error>> {
    let configText = std::fs::read_to_string(CONFIG_PATH)
        .map_err(|e| {
            let msg = format!("YAML find error: {e}");
            msg
        })?;

    let mut yamlValue: yaml_serde::Value =
        yaml_serde::from_str(&configText)
            .map_err(|e| {
                let msg = format!("YAML parse error: {e}");
                msg
            })?;

    yamlValue.apply_merge()
        .map_err(|e| {
            let msg = format!("YAML merge error: {e}");
                msg
        })?;

    let yamlConfig: Config =
        yaml_serde::from_value(yamlValue)
            .map_err(|e| {
                let msg = format!("YAML config error: {e}");
                msg
            })?;
    Ok(yamlConfig)
}
pub fn writeYaml(config: &Config) -> Result<(), String> {
    let yamlText = yaml_serde::to_string(config)
        .map_err(|e| {
            format!("YAML serialization error: {e}")
        })?;

    std::fs::write(CONFIG_PATH, yamlText)
        .map_err(|e| {
            format!("YAML write error: {e}")
        })?;

    Ok(())
}
