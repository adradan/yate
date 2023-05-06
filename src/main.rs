use argh::FromArgs;
use std::{io, time::Duration};

mod app;
mod crossterm_backend;
mod ui;

use crate::crossterm_backend::run;

#[derive(FromArgs)]
#[argh(description = "Test CLI app")]
struct Cli {
    // CLI Options for program
    #[argh(option, default = "250")]
    #[argh(description = "tick rate for app")]
    tick_rate: u64,
}

fn main() -> Result<(), io::Error> {
    let cli: Cli = argh::from_env();
    let tick_rate = Duration::from_millis(cli.tick_rate);
    run(tick_rate)?;

    Ok(())
}
