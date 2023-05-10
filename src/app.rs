use crate::event_handler::{Config, IoEvent};
use crate::tabs::TabsState;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

use serde_json;

pub struct App {
    pub title: String,
    pub quit: bool,
    pub tabs: TabsState,
    pub config: Config,
}

impl App {
    pub fn new_app(title: String) -> Self {
        App {
            title,
            quit: false,
            tabs: TabsState::new(vec!["Tab 1".to_string(), "Tab 2".to_string()]),
            config: Config::new(true),
        }
    }

    pub fn check_event(&mut self, key: KeyEvent) {
        let io_event = self.config.keybinds.get_keybind(&key);
        match io_event {
            IoEvent::NextTab => {
                self.tabs.next();
            }
            IoEvent::PreviousTab => {
                self.tabs.previous();
            }
            IoEvent::QuitApp => {
                self.quit_app();
            }
            IoEvent::Test => {
                self.test();
            }
            IoEvent::Unknown => {
                println!("Unkonnn.");
            }
        }
    }

    fn quit_app(&mut self) {
        self.quit = true;
    }

    fn test(&mut self) {
        let x = serde_json::to_string(&KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        })
        .unwrap();
        let y = serde_json::to_string(&self.config.keybinds).unwrap();
        println!("{}", y);
    }
}
