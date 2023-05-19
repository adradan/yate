use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct NewTabError;

impl Error for NewTabError {}

impl fmt::Display for NewTabError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Error creating a new tab.")
    }
}

#[derive(Clone, Debug)]
pub enum TabType {
    MainMenu,
    Settings,
}

#[derive(Clone, Debug)]
pub struct Tab {
    pub title: String,
    pub tab_type: TabType,
}

#[derive(Clone, Debug)]
pub struct TabsState {
    pub index: usize,
    pub tabs: Vec<Tab>,
    max_tabs: usize,
}

impl TabsState {
    pub fn new() -> TabsState {
        TabsState {
            tabs: Vec::new(),
            index: 0,
            max_tabs: 5,
        }
    }

    pub fn new_tab(&mut self, title: String, tab_type: TabType) -> Result<(), NewTabError> {
        if self.max_tabs.eq(&self.tabs.len()) {
            return Err(NewTabError);
        }
        self.tabs.push(Tab { title, tab_type });
        Ok(())
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.tabs.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.tabs.len() - 1;
        }
    }

    pub fn get_current_type(&self) -> TabType {
        match self.tabs.get(self.index).unwrap().tab_type {
            TabType::MainMenu => TabType::MainMenu,
            TabType::Settings => TabType::Settings,
        }
    }
}
