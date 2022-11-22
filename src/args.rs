extern crate rustop;

use rustop::opts;

pub struct Args {
    pub config: String,
    pub number: i64
}

pub fn parse_args() -> Args {
    let (args, _) = opts! {
        opt config:String;
    }.parse_or_exit();

    Args {
        config: args.config,
        number: 1
    }
}
