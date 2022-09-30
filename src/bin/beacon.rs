// main.rs

use log::*;
use std::{thread, time};
use structopt::StructOpt;

use beacon::*;

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::from_args();
    opts.finish()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));
    debug!("Runtime config:\n{opts:#?}");

    let m = &opts.message;
    loop {
        info!("{m}");
        thread::sleep(time::Duration::new(opts.interval, 0));
    }
}

// EOF
