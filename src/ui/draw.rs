use std::io;

use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, Borders, Tabs},
    Frame,
};

use crate::app::App;
use crate::state::TabType;

pub fn draw(frame: &mut Frame<CrosstermBackend<io::Stdout>>, app: &App) -> () {
    let screen_size = frame.size();

    let vertical_chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .direction(Direction::Vertical)
        .split(screen_size);

    let tab_titles = app
        .tabs_state
        .tabs
        .iter()
        .cloned()
        .map(|t| Spans::from(t.title))
        .collect();

    let tabs = Tabs::new(tab_titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(app.title.to_string()),
        )
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs_state.index);
    frame.render_widget(tabs, vertical_chunks[0]);

    let current_tab = app.tabs_state.get_current_type();
    match current_tab {
        TabType::MainMenu => {
            draw_main_menu(frame, app, vertical_chunks[1]);
        }
        TabType::Settings => {
            draw_settings(frame, app, vertical_chunks[1]);
        }
    }
    draw_command_block(frame, app, vertical_chunks[2]);
}

fn draw_main_menu(frame: &mut Frame<CrosstermBackend<io::Stdout>>, app: &App, chunk: Rect) {
    let block = create_generic_block("Main Menu");
    frame.render_widget(block, chunk);
}

fn draw_settings(frame: &mut Frame<CrosstermBackend<io::Stdout>>, app: &App, chunk: Rect) {
    let block = create_generic_block("Settings");
    frame.render_widget(block, chunk);
}

fn draw_command_block(frame: &mut Frame<CrosstermBackend<io::Stdout>>, app: &App, chunk: Rect) {
    let block = create_generic_block("");
    let command_text = vec![Spans::from(
        app.state_manager.command_state.get_command_string(),
    )];
    let p = Paragraph::new(command_text).block(block);
    frame.render_widget(p, chunk);
}

fn create_generic_block(title: &str) -> Block {
    Block::default().borders(Borders::ALL).title(title)
}
