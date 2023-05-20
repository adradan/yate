use crossterm::event::KeyCode::Char;
use crossterm::event::{KeyCode, KeyEvent};

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
            v if v.starts_with("cd") => CommandEvent::OpenFile,
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
                self.open_file(command_parts);
            }
            CommandEvent::CloseFile => {}
            CommandEvent::QuitApp => {}
            CommandEvent::Unknown => {}
        }
        self.command = "".to_string();
    }

    fn split_command(&self) -> Vec<&str> {
        return self.command.split(" ").collect();
    }

    fn open_file(&self, command_parts: Vec<&str>) {
        // println!("{:?}", command_parts);
        let file_path = command_parts.get(1);

        if let Some(file_path) = file_path {
            // println!("{}", file_path);
        }
    }
}
