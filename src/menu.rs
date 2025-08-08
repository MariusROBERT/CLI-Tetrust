pub const OPTION_LEN: usize = 3;

#[derive(Copy, Clone, PartialEq)]
pub enum Options {
    None,
    New,
    Scores,
    Quit,
}

impl Options {
    pub fn as_str(&self) -> &'static str {
        match self {
            Options::None => "NONE",
            Options::New => "New Game",
            Options::Scores => "Scores (WIP)",
            Options::Quit => "Quit",
        }
    }
}

pub struct Menu {
    options: [Options; OPTION_LEN],
    hovered: Options,
    selected: Options,
}

impl Menu {
    pub fn new() -> Menu {
        Self {
            hovered: Options::New,
            selected: Options::None,
            options: [Options::New, Options::Scores, Options::Quit],
        }
    }

    pub fn move_down(&mut self) {
        match self.hovered {
            Options::New => self.hovered = Options::Scores,
            Options::Scores => self.hovered = Options::Quit,
            Options::Quit => {}
            Options::None => panic!("You shouldn't hover NONE"),
        };
    }

    pub fn move_up(&mut self) {
        match self.hovered {
            Options::New => {}
            Options::Scores => self.hovered = Options::New,
            Options::Quit => self.hovered = Options::Scores,
            Options::None => panic!("You shouldn't hover NONE"),
        };
    }

    pub fn select(&mut self) {
        self.selected = self.hovered;
    }

    pub fn back(&mut self) {
        match self.selected {
            Options::Scores => self.selected = Options::None,
            _ => self.selected = Options::Quit,
        }
    }

    pub fn selected(&self) -> Options {
        self.selected
    }

    pub fn options(&self) -> [Options; 3] {
        self.options
    }

    pub fn hovered(&self) -> Options {
        self.hovered
    }
}
