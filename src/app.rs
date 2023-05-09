use crate::event_handler::{Config, IoEvent, Keys};
use crate::tabs::TabsState;

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

    pub fn check_event(&mut self, key: Keys) {
        let io_event = self.config.keybinds.get_keybind(&key);
        match io_event {
            IoEvent::NextTab => {
                self.tabs.next();
            }
            IoEvent::PreviousTab => {
                self.tabs.previous();
            }
            IoEvent::QuitApp => {
                self.quit = true;
            }
            IoEvent::Unknown => {
                println!("Unkonnn.");
            }
        }
    }
}
