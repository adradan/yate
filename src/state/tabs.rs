#[derive(Clone, Debug)]
pub enum TabType {
    MainMenu,
    Settings,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct Tab {
    pub title: String,
    tab_type: TabType,
}

impl Tab {
    pub fn new(tab_type: TabType, title: String) -> Self {
        Tab { title, tab_type }
    }

    pub fn get_title(&self) -> &String {
        return &self.title;
    }

    pub fn get_tab_type(&self) -> &TabType {
        return &self.tab_type;
    }
}

#[derive(Clone, Debug)]
pub struct TabsState {
    pub tabs: Vec<Tab>,
    pub index: usize,
}

impl TabsState {
    pub fn new() -> Self {
        TabsState {
            tabs: Vec::new(),
            index: 0,
        }
    }

    pub fn next_tab(&mut self) {
        self.index = (self.index + 1) % self.tabs.len();
    }

    pub fn previous_tab(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.tabs.len() - 1;
        }
    }

    pub fn tab_index(&self) -> usize {
        return self.index;
    }

    pub fn tab_info(&self) -> Option<&Tab> {
        self.tabs.get(self.index)
    }

    pub fn tab_title(&self) -> String {
        self.tabs.get(self.index).unwrap().title.to_string()
    }

    pub fn add_tab(&mut self, title: String, tab_type: TabType) {
        let new_tab = Tab::new(tab_type, title);
        self.tabs.push(new_tab);
    }

    pub fn get_tabs(&self) -> &Vec<Tab> {
        return &self.tabs;
    }
}
