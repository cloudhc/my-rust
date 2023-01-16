mod args;
mod logger;
mod mydb;
mod rpc;
mod yaml;

use anyhow::{Context, Result};
use args::Args;
use yaml::Config;

fn init_rsapp() -> Result<(Args, Config)> {
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
        mydb::load_dbsettings(&config.settings_uri.as_str())?;
    }

    log::info!("running completely.");

    if config.rpc_enabled {
        rpc::new_server(&config.rpc_address, config.rpc_port, config.rpc_allow_cors)?.wait();
    }

    Ok(())
}