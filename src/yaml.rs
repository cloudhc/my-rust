extern crate yaml_rust;

use std::fs::File;
use std::io::prelude::*;
use yaml_rust::YamlLoader;

pub struct Config {
    pub debug_level: i64,
    pub prefix_path: String,

    pub settings_enabled: bool,
    pub settings_uri: String,

    pub rpc_enabled: bool,
    pub rpc_allow_cors: bool,
    pub rpc_address: String,
    pub rpc_port: u16,
}

pub fn load_from_file(path: &str) -> Config {
    let mut f = File::open(path).expect("Error while openig file");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong while reading the file");

    let docs = YamlLoader::load_from_str(&mut contents).unwrap();
    let doc = &docs[0];

    Config {
        debug_level: doc["debug"]["level"].as_i64().unwrap_or(1),
        prefix_path: String::from(doc["paths"]["prefix"].as_str().unwrap_or("/opt/xabyss/css")),

        settings_enabled: doc["settings"]["enabled"].as_bool().unwrap_or(false),
        settings_uri: String::from(doc["settings"]["database"].as_str().unwrap()),

        rpc_enabled: doc["control"]["enabled"].as_bool().unwrap_or(false),
        rpc_allow_cors: doc["control"]["allow-cors"].as_bool().unwrap_or(false),
        rpc_address: String::from(doc["control"]["listen-address"].as_str().unwrap_or("127.0.0.1")),
        rpc_port: doc["control"]["listen-port"].as_i64().unwrap() as u16,
    }
}
