use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use serde::{Deserialize, Serialize};

use super::IoEvent;

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Keybinds {
    #[serde_as(as = "HashMap<serde_with::json::JsonString, _>")]
    keybinds: HashMap<KeyEvent, IoEvent>,
}

fn create_key_event(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
    KeyEvent {
        code,
        modifiers,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    }
}

impl Keybinds {
    // Note: Ctrl + Shift + Char keybinds don't seem to work (at least natively), Ctrl will overwrite the shift.
    pub fn new(create_default_config: bool) -> Self {
        let keybinds: HashMap<KeyEvent, IoEvent> = HashMap::new();
        if create_default_config {
            Keybinds::default_config()
        } else {
            Keybinds { keybinds }
        }
    }

    pub fn default_config() -> Keybinds {
        let mut keybinds: HashMap<KeyEvent, IoEvent> = HashMap::new();
        keybinds.insert(
            create_key_event(KeyCode::Left, KeyModifiers::NONE),
            IoEvent::PreviousTab,
        );
        keybinds.insert(
            create_key_event(KeyCode::Right, KeyModifiers::NONE),
            IoEvent::NextTab,
        );
        keybinds.insert(
            create_key_event(KeyCode::Char('q'), KeyModifiers::CONTROL),
            IoEvent::QuitApp,
        );
        keybinds.insert(
            create_key_event(KeyCode::Char('t'), KeyModifiers::NONE),
            IoEvent::Test,
        );
        Keybinds { keybinds }
    }

    pub fn get_keybind(&self, key: &KeyEvent) -> &IoEvent {
        // let x = serde_json::to_string(&key).unwrap();
        // println!("{}", x);
        if let Some(event) = self.keybinds.get(key) {
            event
        } else {
            &IoEvent::Unknown
        }
    }

    pub fn set_keybind(&mut self, old_key: &KeyEvent, new_key: KeyEvent, event: IoEvent) {
        if let Some(_) = self.keybinds.remove(old_key) {
            self.keybinds.insert(new_key, event);
        } else {
            self.keybinds.insert(new_key, event);
        }
    }
}

pub struct Config {
    pub keybinds: Keybinds,
}

impl Config {
    // TODO: Addition of saving configs
    pub fn new(default_config: bool) -> Self {
        if default_config {
            Config {
                keybinds: Keybinds::new(true),
            }
        } else {
            Config {
                keybinds: Keybinds::new(true),
            }
        }
    }
}
