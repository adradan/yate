use crossterm::event::KeyCode::Char;
use crossterm::event::{KeyCode, KeyEvent};
use std::path::Path;

#[derive(Debug)]
enum CommandEvent {
    OpenFile,
    CloseFile,
    QuitApp,
    Unknown,
}

impl From<&String> for CommandEvent {
    fn from(value: &String) -> Self {
        // Removes the : character
        match &value[1..] {
            v if v.starts_with("open") => CommandEvent::OpenFile,
            v if v.to_string() == "sc" => {
                // Save and Close
                CommandEvent::CloseFile
            }
            v if v.to_string() == "q" => CommandEvent::QuitApp,
            _ => CommandEvent::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct CommandState {
    pub command: String,
    pub finished: bool,
}

impl CommandState {
    pub fn new() -> Self {
        CommandState {
            command: String::new(),
            finished: true,
        }
    }

    pub fn get_command_string(&self) -> String {
        self.command.to_string()
    }

    pub fn is_finished(&self) -> &bool {
        &self.finished
    }

    pub fn add_to_command(&mut self, c: char) {
        let new_command = format!("{}{}", self.command, c);
        self.command = new_command;
    }

    fn remove_from_command(&mut self) {
        if self.command.len() == 1 {
            return;
        }
        let new_command = self.command[..self.command.len() - 1].to_string();
        self.command = new_command;
    }

    pub fn compose_command(&mut self, key: KeyEvent) {
        match key {
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => {
                self.execute_command();
            }
            KeyEvent { code: Char(c), .. } => {
                self.add_to_command(c);
            }
            KeyEvent {
                code: KeyCode::Backspace,
                ..
            } => {
                self.remove_from_command();
            }
            _ => {}
        }
    }

    pub fn start_command(&mut self) {
        self.finished = false;
        self.command = String::from(':');
    }

    pub fn end_command(&mut self) {
        // println!("{}", self.command);
        self.finished = true;
    }

    pub fn execute_command(&mut self) {
        self.finished = true;
        let command_parts = self.split_command();
        // println!("{}", self.command);
        let command_event = CommandEvent::from(&self.command);
        match command_event {
            CommandEvent::OpenFile => {
                self.open_file(command_parts.1);
            }
            CommandEvent::CloseFile => {}
            CommandEvent::QuitApp => {}
            CommandEvent::Unknown => {}
        }
        self.command = "".to_string();
    }

    fn split_command(&self) -> (&str, &str) {
        let default_tuple = ("", "");
        return *self.command.split_once(" ").get_or_insert(default_tuple);
    }

    fn open_file(&self, file_path: &str) {
        if !file_path.is_empty() && Path::new(&file_path).exists() {
            // Open and draw
            return;
        }
    }
}
