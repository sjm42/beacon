// main.rs

use clap::Parser;
use log::*;
use std::{
    io::{self, Write},
    thread, time,
};

use beacon::*;

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finish()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));
    debug!("Runtime config:\n{opts:#?}");

    let m = &opts.message;
    loop {
        if opts.timestamp {
            info!("{m}");
        } else {
            writeln!(io::stdout(), "{m}")?;
        }
        io::stdout().flush()?;
        thread::sleep(time::Duration::new(opts.interval, 0));
    }
}

// EOF
