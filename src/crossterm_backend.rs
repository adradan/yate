use std::{time::{Duration, Instant}, io};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, Event, self, KeyEventKind, KeyCode}};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{ui, app::App};

pub fn run(tick_rate: Duration) -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let app = App::new_app("SOME TITLE");
    run_app(&mut terminal, app, tick_rate)?;
    
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
        mut app: App,
        tick_rate: Duration
    ) -> Result<(), io::Error>{
    terminal.clear()?;
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|frame| ui::draw(frame, &mut app))?;
        let timeout = tick_rate.checked_sub(last_tick.elapsed()).unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(c) => app.on_key(c),
                        KeyCode::Right => app.on_right(),
                        KeyCode::Left => app.on_left(),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }

        if app.quit {
            return Ok(());
        }
    }
}