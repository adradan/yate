use std::io;
use std::rc::Rc;

use ratatui::layout::Rect;
use ratatui::style::Modifier;
use ratatui::text::Span;
use ratatui::widgets::{Cell, Paragraph, Row, Table};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, Borders, Tabs},
    Frame,
};

use crate::app::App;
use crate::state::{Tab, TabType};

pub fn draw(frame: &mut Frame<CrosstermBackend<io::Stdout>>, app: &App) {
    let screen_size = frame.size();

    let vertical_chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Percentage(80),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .direction(Direction::Vertical)
        .split(screen_size);

    let tab_titles = app
        .tabs
        .get_tabs()
        .iter()
        .cloned()
        .map(|t| {
            let title = t.get_title().to_string();
            Spans::from(title)
        })
        .collect();

    let tabs = Tabs::new(tab_titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(app.title.to_string()),
        )
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.tab_index());

    frame.render_widget(tabs, vertical_chunks[0]);
    if let Some(current_tab) = app.tabs.tab_info() {
        draw_tab(frame, app, current_tab, vertical_chunks);
    }
    // frame.render_widget(block, vertical_chunks[1]);
}

fn create_tab_block<'a>(title: String) -> Block<'a> {
    Block::default().title(title).borders(Borders::ALL)
}

fn draw_tab(
    frame: &mut Frame<CrosstermBackend<io::Stdout>>,
    app: &App,
    tab: &Tab,
    chunks: Rc<[Rect]>,
) {
    match tab.get_tab_type() {
        TabType::MainMenu => {
            draw_main_menu(frame, app, chunks);
        }
        TabType::Settings => {
            draw_settings(frame, app, chunks);
        }
        TabType::Unknown => {}
    }
}

fn draw_main_menu(frame: &mut Frame<CrosstermBackend<io::Stdout>>, app: &App, chunks: Rc<[Rect]>) {
    let tab_title = app.tabs.tab_title();
    let block = create_tab_block(tab_title);
    let text = vec![Spans::from(Span::raw("TESTETTESTST"))];
    let new_block = Block::default().borders(Borders::ALL);

    let p = Paragraph::new(text).block(new_block);

    frame.render_widget(block, chunks[1]);
    // frame.render_widget(p, chunks[2])
}

fn draw_settings(frame: &mut Frame<CrosstermBackend<io::Stdout>>, app: &App, chunks: Rc<[Rect]>) {
    let tab_title = app.tabs.tab_title();
    let block = create_tab_block(tab_title);

    frame.render_widget(block, chunks[1]);
}
