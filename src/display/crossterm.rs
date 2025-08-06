use std::error::Error;
use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::{Backend, CrosstermBackend};

use crate::display::{game_ui, menu_ui};
use crate::menu::{Menu, Options};
use crate::tetris::Tetris;

pub fn run(tick_rate: Duration) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create game and run it
    loop {
        match run_menu(&mut terminal) {
            Ok(true) => break,
            Err(e) => {
                eprintln!("{e:?}");
            }
            _ => {}
        };
        match run_game(&mut terminal, Tetris::new(), tick_rate) {
            Ok(true) => break,
            Err(e) => {
                eprintln!("{e:?}");
            }
            _ => {}
        };
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_game<B: Backend>(
    terminal: &mut Terminal<B>,
    mut game: Tetris,
    tick_rate: Duration,
) -> Result<bool, Box<dyn Error>> {
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|frame| game_ui::draw(frame, &mut game))?;

        if game.is_lost() {
            return Ok(false);
        }

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if !event::poll(timeout)? {
            game.on_tick();
            last_tick = Instant::now();
            continue;
        }
        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Esc => return Ok(true),

                KeyCode::Char('q') => game.rotate_counter_clockwise(),
                KeyCode::Char('e') => game.rotate_clockwise(),

                KeyCode::Char('a') => game.r#move([0, -1]),
                KeyCode::Char('s') => game.r#move([1, 0]),
                KeyCode::Char('d') => game.r#move([0, 1]),
                _ => {}
            }
        }
    }
}

fn run_menu<B: Backend>(terminal: &mut Terminal<B>) -> Result<bool, Box<dyn Error>> {
    let mut menu = Menu::new();

    loop {
        terminal.draw(|frame| menu_ui::draw(frame, &menu))?;

        match menu.get_selected() {
            Options::QUIT => return Ok(true),
            Options::NEW => return Ok(false),
            _ => { /* Don't care */ }
        };

        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Esc => return Ok(true),
                KeyCode::Up | KeyCode::Char('w') => menu.move_up(),
                KeyCode::Down | KeyCode::Char('s') => menu.move_down(),
                KeyCode::Enter => menu.select(),
                _ => {}
            }
        }
    }
}
