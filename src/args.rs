use rustop::opts;

#[derive(Debug)]
pub struct Args {
    pub config: String,
    pub number: i64
}

impl Args {
    pub fn build() -> Result<Args, &'static str> {
        let Ok((args, _)) = opts! {
            opt config:Option<String>;
        }.parse() else {
            return Err("Usage: xa-rust -c etc/main.yaml");
        };

        Ok(Args {
            config: args.config.unwrap(),
            number: 1
        })
    }
}
