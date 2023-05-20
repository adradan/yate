use crate::event_handler::{Config, IoEvent};
use crate::state::{CommandState, StateManager, TabType, TabsState};
use crossterm::event::KeyCode::Char;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

use serde_json;

pub struct App {
    pub title: String,
    pub quit: bool,
    pub tabs_state: TabsState,
    pub config: Config,
    pub state_manager: StateManager,
}

impl App {
    pub fn new_app(title: String) -> Self {
        let mut tabs_state = TabsState::new();
        tabs_state
            .new_tab("Main Menu".to_string(), TabType::MainMenu)
            .unwrap();
        tabs_state
            .new_tab("Settings".to_string(), TabType::Settings)
            .unwrap();

        App {
            title,
            quit: false,
            tabs_state,
            config: Config::new(true),
            state_manager: StateManager::new(),
        }
    }

    pub fn check_event(&mut self, key: KeyEvent) {
        let io_event = self.config.keybinds.get_keybind(key);
        let handle_keybinds = self.state_manager.handle_state(key);
        if !handle_keybinds {
            return;
        }
        // Customizable Keybinds
        match io_event {
            IoEvent::NextTab => {
                self.tabs_state.next();
            }
            IoEvent::PreviousTab => {
                self.tabs_state.previous();
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
