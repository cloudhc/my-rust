extern crate rustop;

use rustop::opts;

pub struct Args {
    pub config: String,
    pub number: i64
}

impl Args {
    pub fn build() -> Result<Args, &'static str> {
        let (args, _) = opts! {
            opt config:Option<String>;
        }.parse_or_exit();

        if args.config.is_none() {
            return Err("Usage: xa-rust -c etc/main.yaml");
        }

        Ok(Args {
            config: args.config.unwrap(),
            number: 1
        })
    }
}
