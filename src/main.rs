mod args;
mod logger;
mod mydb;
mod rpc;
mod yaml;

use anyhow::{Context, Result};
use args::Args;
use signal_hook::{
    consts::{SIGINT, SIGTERM},
    iterator::Signals,
    low_level,
};
use std::thread;
use yaml::Config;

fn init_rsapp() -> Result<(Args, Config)> {
    let mut signals = Signals::new(&[SIGINT, SIGTERM])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            eprintln!("Received signal {sig:?}");
            if let Err(e) = low_level::emulate_default_handler(sig) {
                eprintln!("Failed to handle default emulating with {e}.");
            }
        }
    });

    let args: Args = Args::build().context("Usage: xa-rust -c etc/main.yaml")?;
    let config = Config::load_from_file(args.config.as_str())?;
    let _ = logger::init_logger(&config.output_log_path)?;

    Ok((args, config))
}

fn main() -> Result<()> {
    let (args, config) = init_rsapp()?;

    println!("Args is {args:#?}");
    println!("Config is {config:#?}");

    if config.settings_enabled {
        let setting = mydb::load_dbsettings(&config.settings_uri.as_str())?;

        log::info!("ports : {:?}", setting);

        mydb::fetch_vulnerabilities(&config.settings_uri.as_str())?;
    }

    log::info!("running completely.");

    if config.rpc_enabled {
        rpc::new_server(&config.rpc_address, config.rpc_port, config.rpc_allow_cors)?.wait();
    }

    Ok(())
}
