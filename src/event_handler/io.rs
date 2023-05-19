use std::{fmt, sync::mpsc, thread, time::Duration};

use crossterm::event;
use crossterm::event::KeyEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum IoEvent {
    NextTab,
    PreviousTab,
    QuitApp,
    Test,
    Unknown,
}

impl fmt::Display for IoEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            IoEvent::NextTab => write!(f, "Next"),
            IoEvent::PreviousTab => write!(f, "Prev"),
            IoEvent::QuitApp => write!(f, "Quit"),
            IoEvent::Test => write!(f, "Test"),
            IoEvent::Unknown => write!(f, "Unknown"),
        }
    }
}

impl From<&IoEvent> for IoEvent {
    fn from(io_event: &IoEvent) -> Self {
        match io_event {
            IoEvent::NextTab => IoEvent::NextTab,
            IoEvent::PreviousTab => IoEvent::PreviousTab,
            IoEvent::QuitApp => IoEvent::QuitApp,
            IoEvent::Test => IoEvent::Test,
            IoEvent::Unknown => IoEvent::Unknown,
        }
    }
}

pub struct Messages {
    rx: mpsc::Receiver<KeyEvent>,
    _tx: mpsc::Sender<KeyEvent>,
}

impl Messages {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::channel::<KeyEvent>();

        let event_tx = tx.clone();

        thread::spawn(move || loop {
            if event::poll(tick_rate).unwrap() {
                if let event::Event::Key(key) = event::read().unwrap() {
                    if event::KeyEventKind::Press != key.kind {
                        continue;
                    }
                    // let x = serde_json::to_string(&key).unwrap();
                    // println!("{}", x);
                    event_tx.send(key).unwrap();
                }
            }
        });

        Messages { rx, _tx: tx }
    }

    pub fn get_event(&self) -> Result<KeyEvent, mpsc::RecvError> {
        self.rx.recv()
    }
}

#[cfg(test)]
mod tests {
    use super::IoEvent;

    #[test]
    fn from_borrowed_io() {
        let event = IoEvent::from(&IoEvent::QuitApp);
        assert_eq!(event, IoEvent::QuitApp);
    }
}
