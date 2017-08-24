#![recursion_limit = "1024"]
use error_chain::ChainedError;

#[macro_use]
extern crate error_chain;

mod errors {
    error_chain!{}
}

use errors::*;

quick_main!(run);

fn run() -> Result<()> {
    use std::fs::File;

    // This operation will fail
    File::open("tretrete").chain_err(
        || "unable to open tretrete file",
    )?;

    Ok(())
}
