use std::{error::Error, io};

use crossterm::event;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{Event, KeyCode, EnableMouseCapture, DisableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
    },
    Terminal,
};


mod app;
mod ui;

use crate::{
    app::{App, CurrScreen},
    ui::ui,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let mut app = App::new();
    app.load_from_file();
    let _ = run_app(&mut terminal, &mut app);

    // Clean up terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        // Draw the app
        terminal.draw(|f| ui(f, app))?;

        // Handle input
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }

            match app.curr_screen {
                CurrScreen::Main => match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Char('e') => app.enter_edit_mode(),
                    KeyCode::Char('h') | KeyCode::Left  => app.curr_date = app.curr_date.previous_day().unwrap(),
                    KeyCode::Char('l') | KeyCode::Right => app.curr_date = app.curr_date.next_day().unwrap(),
                    KeyCode::Char('j') | KeyCode::Up    => app.curr_date = app.curr_date.prev_occurrence(app.curr_date.weekday()),
                    KeyCode::Char('k') | KeyCode::Down  => app.curr_date = app.curr_date.next_occurrence(app.curr_date.weekday()),
                    KeyCode::Char('>') => app.next_zoom(),
                    KeyCode::Char('<') => app.prev_zoom(),
                    _ => {},
                }
                CurrScreen::Editing => match key.code {
                    KeyCode::Esc => app.curr_screen = CurrScreen::DiscardChanges,
                    KeyCode::Enter => {app.enter_main_mode(); app.save_to_file();},
                    KeyCode::Char(c) => app.type_char(c),
                    KeyCode::Backspace => app.remove_char(),
                    KeyCode::Tab => app.cycle_edit_value(),
                    KeyCode::Left => app.prev_cursor(),
                    KeyCode::Right => app.next_cursor(),
                    _ => {},
                }
                CurrScreen::DiscardChanges => match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => app.curr_screen = CurrScreen::Main,
                    KeyCode::Char('n') | KeyCode::Char('N')=> app.curr_screen = CurrScreen::Editing,
                    _ => {},
                }
            }
        }
    }

    Ok(true)
}
