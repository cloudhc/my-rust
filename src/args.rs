use anyhow::Result;
use rustop::opts;

#[derive(Debug)]
pub struct Args {
    pub config: String,
    pub number: i64
}

impl Args {
    pub fn build() -> Result<Args> {
        let (args, _) = opts! {
            opt config:Option<String>;
        }.parse_or_exit();

        if args.config.is_none() {
            anyhow::bail!("Failed to parse cli argument.");
        }

        Ok(Args {
            config: args.config.unwrap(),
            number: 1
        })
    }
}
