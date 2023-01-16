use anyhow::{Context, Result};
use std::fs;
use yaml_rust::YamlLoader;

#[derive(Debug)]
pub struct Config {
    pub debug_level: i64,
    pub prefix_path: String,

    pub output_log_path: String,

    pub settings_enabled: bool,
    pub settings_uri: String,

    pub rpc_enabled: bool,
    pub rpc_allow_cors: bool,
    pub rpc_address: String,
    pub rpc_port: u16,
}

impl Config {
    pub fn load_from_file(path: &str) -> Result<Config> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("Couldn't find out from {path}"))?;

        let root_docs = YamlLoader::load_from_str(&contents)
            .context("Failed to laod YAML file")?;

        let docs = root_docs.get(0)
            .context("Node is empty!!!.")?;

        Ok(Config {
            debug_level: docs["debug"]["level"].as_i64().unwrap_or(1),
            prefix_path: String::from(docs["paths"]["prefix"].as_str().unwrap_or("/opt/xabyss/css")),

            output_log_path: String::from(docs["output"]["log-path"].as_str().unwrap_or("log/main.log")),

            settings_enabled: docs["settings"]["enabled"].as_bool().unwrap_or(false),
            settings_uri: String::from(docs["settings"]["database"].as_str().unwrap()),

            rpc_enabled: docs["control"]["enabled"].as_bool().unwrap_or(false),
            rpc_allow_cors: docs["control"]["allow-cors"].as_bool().unwrap_or(false),
            rpc_address: String::from(docs["control"]["listen-address"].as_str().unwrap_or("127.0.0.1")),
            rpc_port: docs["control"]["listen-port"].as_i64().unwrap() as u16,
        })
    }
}