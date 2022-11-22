mod args;
mod evt;
mod logger;
mod rpc;
mod yaml;

use log::info;

fn main() {
    let args = args::parse_args();
    let options = yaml::load_from_file(args.config.as_str());

    logger::init_logger("log/rust.log");

    println!("debug_level: {}", options.debug_level);
    println!("prefix_path: {}", options.prefix_path);
    println!("eanbled: {}", options.rpc_enabled);
    println!("allow-cros: {}", options.rpc_allow_cors);
    println!("listen-address: {}", options.rpc_address);
    println!("listen-port: {}", options.rpc_port);

    info!("Hello, world!");

    let mut event_loop = EventLoop::new().unwrap();
    let timeout = event_loop.timeout_ms(123, 300).unwrap();
    let _ = event_loop.run(&mut MyHandler);

    let server = rpc::new_server(options.rpc_address.as_str(), options.rpc_port, options.rpc_allow_cors);
    server.wait();
}
