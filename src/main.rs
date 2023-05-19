use std::time::Duration;

mod app;
mod crossterm_backend;
mod event_handler;
mod state;
mod ui;

use crate::crossterm_backend::run;

struct Options {
    tick_rate: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_options = Options { tick_rate: 250 };
    let tick_rate = Duration::from_millis(app_options.tick_rate);

    run(tick_rate)?;

    Ok(())
}
