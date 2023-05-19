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
    // Note: Ctrl + Shift + Char keybinds don't work , Ctrl will overwrite the shift.
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
            create_key_event(KeyCode::Left, KeyModifiers::CONTROL),
            IoEvent::PreviousTab,
        );
        keybinds.insert(
            create_key_event(KeyCode::Right, KeyModifiers::CONTROL),
            IoEvent::NextTab,
        );
        keybinds.insert(
            create_key_event(KeyCode::Char('q'), KeyModifiers::CONTROL),
            IoEvent::QuitApp,
        );
        keybinds.insert(
            create_key_event(KeyCode::Char('t'), KeyModifiers::CONTROL),
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

    pub fn overwrite_keybind(&mut self, old_key: &KeyEvent, new_key: KeyEvent, event: IoEvent) {
        if let Some(_) = self.keybinds.remove(old_key) {
            self.keybinds.insert(new_key, event);
        } else {
            self.keybinds.insert(new_key, event);
        }
    }

    pub fn set_keybind(&mut self, new_key: KeyEvent, event: IoEvent) {
        self.keybinds.insert(new_key, event);
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

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyModifiers};

    use crate::event_handler::IoEvent;

    use super::{create_key_event, Config, Keybinds};

    #[test]
    fn will_create_default_config() {
        let config = Config::new(true);
        let previous_tab = config
            .keybinds
            .get_keybind(&create_key_event(KeyCode::Left, KeyModifiers::CONTROL));
        let next_tab = config
            .keybinds
            .get_keybind(&create_key_event(KeyCode::Right, KeyModifiers::CONTROL));
        let quit = config
            .keybinds
            .get_keybind(&create_key_event(KeyCode::Char('q'), KeyModifiers::CONTROL));
        let unknown = config
            .keybinds
            .get_keybind(&create_key_event(KeyCode::Char('p'), KeyModifiers::ALT));
        assert_eq!(previous_tab, &IoEvent::PreviousTab);
        assert_eq!(next_tab, &IoEvent::NextTab);
        assert_eq!(quit, &IoEvent::QuitApp);
        assert_eq!(unknown, &IoEvent::Unknown);
    }

    #[test]
    fn will_get_unknown() {
        let keybinds = Keybinds::new(true);
        let unknown =
            keybinds.get_keybind(&create_key_event(KeyCode::Char('p'), KeyModifiers::ALT));
        assert_eq!(unknown, &IoEvent::Unknown);
    }

    #[test]
    fn will_set_keybind() {
        let mut keybinds = Keybinds::new(true);
        let new_key = create_key_event(KeyCode::Char('p'), KeyModifiers::ALT);
        keybinds.set_keybind(new_key, IoEvent::QuitApp);

        let quit_event = keybinds.get_keybind(&new_key);
        assert_eq!(quit_event, &IoEvent::QuitApp);
    }

    #[test]
    fn will_overwrite_keybind() {
        let mut keybinds = Keybinds::new(true);
        let new_key = create_key_event(KeyCode::Char('p'), KeyModifiers::ALT);
        let old_key = create_key_event(KeyCode::Char('q'), KeyModifiers::CONTROL);
        keybinds.overwrite_keybind(&old_key, new_key, IoEvent::QuitApp);

        let overwritten = keybinds.get_keybind(&old_key);
        let new_keybind = keybinds.get_keybind(&new_key);

        assert_eq!(overwritten, &IoEvent::Unknown);
        assert_eq!(new_keybind, &IoEvent::QuitApp);
    }
}
