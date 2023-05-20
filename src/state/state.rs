use crate::state::CommandState;
use crossterm::event::KeyCode::Char;
use crossterm::event::KeyEvent;

#[derive(Debug)]
pub enum States {
    Command,
    Insert,
    View,
    // Relinquish control back to the app manager, if needed
    App,
}

#[derive(Debug)]
pub struct StateManager {
    current_state: States,
    pub command_state: CommandState,
}

impl StateManager {
    pub fn new() -> Self {
        StateManager {
            current_state: States::App,
            command_state: CommandState::new(),
        }
    }

    pub fn handle_state(&mut self, key: KeyEvent) -> bool {
        // Returns true if app manager should handle the Key Event
        match self.current_state {
            States::Command => {
                self.command_state.compose_command(key);
                if self.command_state.is_finished().eq(&true) {
                    self.current_state = States::App;
                    return true;
                }
                false
            }
            States::Insert => true,
            States::View => true,
            States::App => {
                let enter_command = self.check_command_keybind(key);
                if enter_command {
                    return false;
                }
                // Check other state keybinds
                true
            }
        }
    }

    fn check_command_keybind(&mut self, key: KeyEvent) -> bool {
        match key {
            KeyEvent {
                code: Char(':'), ..
            } => {
                self.command_state.start_command();
                self.current_state = States::Command;
                true
            }
            _ => false,
        }
    }
}
