use anyhow::{Context, Result};
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

pub fn init_logger(path: &str) -> Result<log4rs::Handle> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(path)
        .with_context(|| format!("Couldn't create {path} file."))?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .context("Couldn't set up LevelFilter.")?;

    let handle = log4rs::init_config(config)
        .context("Couldn't make handler.")?;

    Ok(handle)
}