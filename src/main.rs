mod args;
mod mydb;
mod logger;
mod rpc;
mod yaml;

use std::process;
use args::Args;
use yaml::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::build().unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
    let options = Config::load_from_file(args.config.as_str()).unwrap_or_else(|err| {
        println!("Failed on loading yaml file :\n{}", err);
        process::exit(1);
    });

    logger::init_logger("log/rust.log");

    println!("debug_level: {:?}", options.debug_level);
    println!("prefix_path: {:?}", options.prefix_path);
    println!("settings_enabled: {:?}", options.settings_enabled);
    println!("settings_uri: {:?}", options.settings_uri);
    println!("enabled: {:?}", options.rpc_enabled);
    println!("allow-cros: {:?}", options.rpc_allow_cors);
    println!("listen-address: {:?}", options.rpc_address);
    println!("listen-port: {:?}", options.rpc_port);

    if options.settings_enabled {
        if let Err(e) = mydb::load_dbsettings(&options.settings_uri.as_str()) {
            return Err(e);
        }
    }

    if options.rpc_enabled {
        let server = rpc::new_server(&options.rpc_address, options.rpc_port, options.rpc_allow_cors);
        server.wait();
    }

    Ok(())
}
