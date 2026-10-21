use yaml_serde::Value as yamlValue;

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SVG {
    elements        : yamlValue,
    optimization    : yamlValue,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Paths {
    dir     : yamlValue,
    files   : yamlValue,
}

#[derive(serde::Deserialize, Clone)]
// #[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServerOptions {
    pub ms_input        : u64,
    pub voting_period   : u64,
    pub poll_actions    : usize,
    pub polling         : bool,
    pub paused          : bool,
}

#[derive(serde::Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct GamemodeDefaults {
    pub democracy_defaults  : ServerOptions,
    pub anarchy_defaults    : ServerOptions,
    pub pause_defaults      : ServerOptions,
    pub initial_state       : usize,
    // 0 is democracy, 1 is anarchy, 2 is paused
}