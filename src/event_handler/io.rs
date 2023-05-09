use std::{fmt, sync::mpsc, thread, time::Duration};

use crossterm::event;

use super::Keys;

#[derive(Debug)]
pub enum IoEvent {
    NextTab,
    PreviousTab,
    QuitApp,
    Unknown,
}

impl fmt::Display for IoEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            IoEvent::NextTab => write!(f, "Next"),
            IoEvent::PreviousTab => write!(f, "Prev"),
            IoEvent::QuitApp => write!(f, "Quit"),
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
            IoEvent::Unknown => IoEvent::Unknown,
        }
    }
}

pub struct Messages {
    rx: mpsc::Receiver<Keys>,
    _tx: mpsc::Sender<Keys>,
}

impl Messages {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::channel::<Keys>();

        let event_tx = tx.clone();

        thread::spawn(move || loop {
            if event::poll(tick_rate).unwrap() {
                if let event::Event::Key(key) = event::read().unwrap() {
                    if event::KeyEventKind::Press != key.kind {
                        continue;
                    }
                    let key = Keys::from(key);
                    event_tx.send(key).unwrap();
                }
            }
        });

        Messages { rx, _tx: tx }
    }

    pub fn get_event(&self) -> Result<Keys, mpsc::RecvError> {
        self.rx.recv()
    }
}
