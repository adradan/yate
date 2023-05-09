use std::io;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, Borders, Tabs},
    Frame,
};

use crate::app::App;

pub fn draw(frame: &mut Frame<CrosstermBackend<io::Stdout>>, app: &App) -> () {
    let screen_size = frame.size();

    let vertical_chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .direction(Direction::Vertical)
        .split(screen_size);

    let block = Block::default().title("SOME BLOCK").borders(Borders::ALL);

    let tab_titles = app.tabs.titles.iter().cloned().map(Spans::from).collect();

    let tabs = Tabs::new(tab_titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(app.title.to_string()),
        )
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);

    frame.render_widget(tabs, vertical_chunks[0]);
    frame.render_widget(block, vertical_chunks[1]);
}
