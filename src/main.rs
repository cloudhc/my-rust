mod args;
mod mydb;
mod logger;
mod rpc;
mod yaml;

fn main() {
    let args = args::parse_args();
    let options = yaml::load_from_file(args.config.as_str());

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
        mydb::load_dbsettings(&options.settings_uri.as_str());
    }
    if options.rpc_enabled {
        let server = rpc::new_server(&options.rpc_address, options.rpc_port, options.rpc_allow_cors);
        server.wait();
    }

    log::info!("Hello, rust world!");
}
