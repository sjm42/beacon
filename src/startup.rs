// startup.rs

use env_logger::{Builder, Target};
use log::*;
use std::env;
use structopt::StructOpt;

#[derive(Clone, Debug, Default, StructOpt)]
pub struct OptsCommon {
    #[structopt(long, short)]
    pub timestamp: bool,
    #[structopt(long, short, default_value = "ping")]
    pub message: String,
    #[structopt(long, short, default_value = "60")]
    pub interval: u64,
}

impl OptsCommon {
    pub fn finish(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
    pub fn start_pgm(&self, name: &str) {
        Builder::new()
            .target(Target::Stdout)
            .filter_module(env!("CARGO_PKG_NAME"), LevelFilter::Info)
            .filter_module(name, LevelFilter::Info)
            .format_timestamp_secs()
            .init();
    }
}
// EOF
