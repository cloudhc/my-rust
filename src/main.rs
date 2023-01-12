mod args;
mod logger;
mod mydb;
mod rpc;
mod yaml;

use args::Args;
use yaml::Config;

use std::process;

fn init_rsapp() -> Result<(Args, Config, log4rs::Handle), Box<dyn std::error::Error>> {
    let args: Args = Args::build()?;
    let options = Config::load_from_file(args.config.as_str())?;
    let handle = logger::init_logger(&options.output_log_path)?;

    if options.settings_enabled {
        mydb::load_dbsettings(&options.settings_uri.as_str())?;
    }

    Ok((args, options, handle))
}

fn main() {
    let (args, options, _logger) = init_rsapp()
        .unwrap_or_else(|err | {
        println!("Error is {err}");
        process::exit(1);
    });

    println!("Args is {args:#?}");
    println!("config is {options:#?}");

    if options.rpc_enabled {
        let server = rpc::new_server(&options.rpc_address, options.rpc_port, options.rpc_allow_cors); 
        server.wait();
    }
}
