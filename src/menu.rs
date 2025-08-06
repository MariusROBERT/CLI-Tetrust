pub const OPTION_LEN: usize = 3;

#[derive(Copy, Clone, PartialEq)]
pub enum Options {
    NONE,
    NEW,
    SCORES,
    QUIT,
}

impl Options {
    pub fn as_str(&self) -> &'static str {
        match self {
            Options::NONE => "NONE",
            Options::NEW => "New Game",
            Options::SCORES => "Scores",
            Options::QUIT => "Quit",
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
            hovered: Options::NEW,
            selected: Options::NONE,
            options: [Options::NEW, Options::SCORES, Options::QUIT],
        }
    }

    pub fn move_down(&mut self) {
        match self.hovered {
            Options::NEW => self.hovered = Options::SCORES,
            Options::SCORES => self.hovered = Options::QUIT,
            Options::QUIT => {}
            Options::NONE => panic!("You shouldn't hover NONE"),
        };
    }

    pub fn move_up(&mut self) {
        match self.hovered {
            Options::NEW => {}
            Options::SCORES => self.hovered = Options::NEW,
            Options::QUIT => self.hovered = Options::SCORES,
            Options::NONE => panic!("You shouldn't hover NONE"),
        };
    }

    pub fn select(&mut self) {
        self.selected = self.hovered;
    }

    pub fn back(&mut self) {
        self.selected = Options::NONE;
    }

    pub fn get_selected(&self) -> Options {
        self.selected
    }

    pub fn get_options(&self) -> [Options; 3] {
        self.options
    }

    pub fn get_hovered(&self) -> Options {
        self.hovered
    }
}
