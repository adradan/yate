use std::io;

use ratatui::{Frame, backend::CrosstermBackend, widgets::{Block, Borders, Tabs}, text::Spans, style::{Style, Color}, layout::{Layout, Constraint}};

use crate::app::App;

pub fn draw(frame: &mut Frame<CrosstermBackend<io::Stdout>>, app: &mut App) -> () {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(frame.size());

    let screen_size = frame.size();
    let block = Block::default().title("SOME BLOCK").borders(Borders::ALL);

    let tab_titles = app.tabs.titles.iter().cloned().map(Spans::from).collect();

    let tabs = Tabs::new(tab_titles)
            .block(Block::default().borders(Borders::ALL).title(app.title))
            .highlight_style(Style::default().fg(Color::Yellow))
            .select(app.tabs.index);
    frame.render_widget(tabs, chunks[0]);
    frame.render_widget(block, screen_size);
}