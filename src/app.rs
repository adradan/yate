use crate::event_handler::{Config, IoEvent};
use crate::state::{TabType, TabsState};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

pub struct App {
    pub title: String,
    pub quit: bool,
    pub tabs: TabsState,
    pub config: Config,
}

impl App {
    pub fn new_app(title: String) -> Self {
        let mut tabs_state = TabsState::new();
        tabs_state.add_tab("Main Menu".to_string(), TabType::MainMenu);
        tabs_state.add_tab("Settings".to_string(), TabType::Settings);
        App {
            title,
            quit: false,
            tabs: tabs_state,
            config: Config::new(true),
        }
    }

    pub fn check_event(&mut self, key: KeyEvent) {
        let io_event = self.config.keybinds.get_keybind(&key);
        match io_event {
            IoEvent::NextTab => {
                self.tabs.next_tab();
            }
            IoEvent::PreviousTab => {
                self.tabs.previous_tab();
            }
            IoEvent::QuitApp => {
                self.quit_app();
            }
            IoEvent::Test => {
                self.test();
            }
            IoEvent::Unknown => {}
        }
    }

    fn quit_app(&mut self) {
        self.quit = true;
    }

    fn test(&mut self) {
        let _x = serde_json::to_string(&KeyEvent {
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
