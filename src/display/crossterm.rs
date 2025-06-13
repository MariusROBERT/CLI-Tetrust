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

use crate::tetris::Tetris;
use crate::display::ui;

pub fn run(tick_rate: Duration, enhanced_graphics: bool) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout /*, EnterAlternateScreen*/ /*, EnableMouseCapture*/
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    // let app = App::new("Crossterm Demo", enhanced_graphics);
    // let app_result = run_app(&mut terminal, app, tick_rate);
    let tetris = Tetris::new();
    let app_result = run_app(&mut terminal, tetris, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = app_result {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut game: Tetris,
    tick_rate: Duration,
) -> Result<(), Box<dyn Error>> {
    let mut last_tick = Instant::now();
    let mut should_quit = false;

    while !should_quit {
        terminal.draw(|frame| ui::draw(frame, &mut game))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if !event::poll(timeout)? {
            // app.on_tick();
            last_tick = Instant::now();
            continue;
        }
        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                // KeyCode::Char('h') | KeyCode::Left => app.on_left(),
                // KeyCode::Char('j') | KeyCode::Down => app.on_down(),
                // KeyCode::Char('k') | KeyCode::Up => app.on_up(),
                // KeyCode::Char('l') | KeyCode::Right => app.on_right(),
                // KeyCode::Char(c) => app.on_key(c),
                // KeyCode::Char('q') => {
                //     should_quit = true;
                // }
                KeyCode::Esc => {
                    should_quit = true;
                }
                _ => {}
            }
        }
    }
    Ok(())
}
