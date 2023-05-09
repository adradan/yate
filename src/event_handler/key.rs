use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::IoEvent;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Keys {
    Left,
    Right,
    Char(char),
    Alt(char),
    Ctrl(char),
    Shift(char),
    Unknown,
}

impl From<KeyEvent> for Keys {
    fn from(key_event: KeyEvent) -> Self {
        match key_event {
            // TODO: Ctrl + Non-Char keys
            // TODO: FN Keys
            KeyEvent {
                code: KeyCode::Left,
                ..
            } => Keys::Left,
            KeyEvent {
                code: KeyCode::Right,
                ..
            } => Keys::Right,
            // Modifiers first then chars
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::ALT,
                ..
            } => Keys::Alt(c),
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => Keys::Ctrl(c),
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::SHIFT,
                ..
            } => Keys::Shift(c),
            KeyEvent {
                code: KeyCode::Char(c),
                ..
            } => Keys::Char(c),
            _ => Keys::Unknown,
        }
    }
}

pub struct Keybinds {
    keybinds: HashMap<Keys, IoEvent>,
}

impl Keybinds {
    pub fn new(create_default_config: bool) -> Self {
        let keybinds: HashMap<Keys, IoEvent> = HashMap::new();
        if create_default_config {
            Keybinds::default_config()
        } else {
            Keybinds { keybinds }
        }
    }

    pub fn default_config() -> Keybinds {
        let mut keybinds: HashMap<Keys, IoEvent> = HashMap::new();
        keybinds.insert(Keys::Left, IoEvent::PreviousTab);
        keybinds.insert(Keys::Right, IoEvent::NextTab);
        keybinds.insert(Keys::Char('q'), IoEvent::QuitApp);
        Keybinds { keybinds }
    }

    pub fn get_keybind(&self, key: &Keys) -> &IoEvent {
        if let Some(event) = self.keybinds.get(key) {
            event
        } else {
            &IoEvent::Unknown
        }
    }

    pub fn set_keybind(&mut self, old_key: &Keys, new_key: Keys, event: IoEvent) {
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
