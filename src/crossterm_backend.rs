use std::{io, time::Duration};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{app::App, event_handler::Messages, ui};

pub fn run(tick_rate: Duration) -> Result<(), io::Error> {
    let mut app = App::new_app("YATE".to_string());

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    run_app(&mut terminal, &mut app, tick_rate)?;

    // On close behavior
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    tick_rate: Duration,
) -> Result<(), io::Error> {
    terminal.clear()?;

    let messages = Messages::new(tick_rate);

    loop {
        terminal.draw(|frame| ui::draw(frame, app))?;

        app.check_event(messages.get_event().unwrap());

        if app.quit {
            return Ok(());
        }
    }
}
