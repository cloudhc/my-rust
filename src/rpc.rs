extern crate jsonrpc_http_server;

use jsonrpc_http_server::*;
use jsonrpc_http_server::jsonrpc_core::*;

pub fn new_server(address: &String, port: u16, allow_cors: bool) -> Server {
    let mut io = IoHandler::default();
	io.add_method("say_hello", |_: Params| {
		Ok(Value::String("hello".into()))
	});

    let server = ServerBuilder::new(io)
            .cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Null]))
            .start_http(&format!("{address}:{port}").parse().unwrap())
            .expect("Unable to start RPC server");

    return server;
}
