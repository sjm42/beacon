// main.rs

use log::*;
use std::{
    io::{self, Write},
    thread, time,
};
use structopt::StructOpt;

use beacon::*;

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::from_args();
    opts.finish()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));
    debug!("Runtime config:\n{opts:#?}");

    let m = &opts.message;
    loop {
        if opts.timestamp {
            info!("{m}");
        } else {
            write!(io::stdout(), "{m}\n")?;
        }
        io::stdout().flush()?;
        thread::sleep(time::Duration::new(opts.interval, 0));
    }
}

// EOF
