use std::{error::Error, io};
use std::time::Instant;

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

use time::{Date, Month};


mod app;
mod ui;

use crate::{
    app::{App, CurrScreen, Entry},
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
    let _ = run_app(&mut terminal, &mut app);

    // Clean up terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    app.save_entry(Entry {
        content: String::from("This is a test entry while we code the UI"),
        weight_kg: Some(60.0),
        waist_cm: Some(89.0),
        date: Date::from_calendar_date(2024, Month::December, 12).unwrap(),
    });
    app.save_entry(Entry {
        content: String::from("This is a test entry while we code the UI"),
        weight_kg: Some(65.0),
        waist_cm: Some(89.0),
        date: Date::from_calendar_date(2024, Month::December, 13).unwrap(),
    });
    app.save_entry(Entry {
        content: String::from("This is a test entry while we code the UI"),
        weight_kg: Some(63.0),
        waist_cm: Some(89.0),
        date: Date::from_calendar_date(2024, Month::December, 14).unwrap(),
    });
    app.save_entry(Entry {
        content: String::from("This is a test entry while we code the UI"),
        weight_kg: Some(68.0),
        waist_cm: Some(89.0),
        date: Date::from_calendar_date(2024, Month::December, 15).unwrap(),
    });
    app.save_entry(Entry {
        content: String::from("This is a test entry while we code the UI"),
        weight_kg: Some(66.0),
        waist_cm: Some(89.0),
        date: Date::from_calendar_date(2024, Month::December, 16).unwrap(),
    });
    app.save_entry(Entry {
        content: String::from("This is a test entry while we code the UI"),
        weight_kg: Some(71.0),
        waist_cm: Some(89.0),
        date: Date::from_calendar_date(2024, Month::December, 17).unwrap(),
    });
    app.save_entry(Entry {
        content: String::from("This is a test entry while we code the UI"),
        weight_kg: Some(69.0),
        waist_cm: Some(89.0),
        date: Date::from_calendar_date(2024, Month::December, 18).unwrap(),
    });
    app.save_entry(Entry {
        content: String::from("This is a test entry while we code the UI"),
        weight_kg: Some(74.0),
        waist_cm: Some(89.0),
        date: Date::from_calendar_date(2024, Month::December, 19).unwrap(),
    });
    app.save_entry(Entry {
        content: String::from("This is a test entry while we code the UI"),
        weight_kg: Some(72.0),
        waist_cm: Some(89.0),
        date: Date::from_calendar_date(2024, Month::December, 20).unwrap(),
    });
    app.save_entry(Entry {
        content: String::from("This is a test entry while we code the UI"),
        weight_kg: Some(77.0),
        waist_cm: Some(88.0),
        date: Date::from_calendar_date(2024, Month::December, 21).unwrap(),
    });
    app.save_entry(Entry {
        content: String::from("This is a test entry while we code the UI"),
        weight_kg: Some(75.0),
        waist_cm: Some(84.0),
        date: Date::from_calendar_date(2024, Month::December, 22).unwrap(),
    });
    app.save_entry(Entry {
        content: String::from("This is a test entry while we code the UI"),
        weight_kg: Some(80.0),
        waist_cm: Some(86.0),
        date: Date::from_calendar_date(2024, Month::December, 23).unwrap(),
    });
    app.save_entry(Entry {
        content: String::from("This is a test entry while we code the UI"),
        weight_kg: None,
        waist_cm: None,
        date: Date::from_calendar_date(2024, Month::December, 24).unwrap(),
    });
    app.save_entry(Entry {
        content: String::from("This is a test entry while we code the UI, and I have just made it long enough so that it needs to wrap and therefore I can see the behaviour"),
        weight_kg: Some(83.0),
        waist_cm: Some(87.0),
        date: Date::from_calendar_date(2024, Month::December, 25).unwrap(),
    });

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
                    _ => {},
                }
                CurrScreen::Editing => match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Enter => app.enter_main_mode(),
                    KeyCode::Char(c) => app.type_char(c),
                    KeyCode::Backspace => app.remove_char(),
                    KeyCode::Tab => app.cycle_edit_value(),
                    KeyCode::Left => app.prev_cursor(),
                    KeyCode::Right => app.next_cursor(),
                    _ => {},
                }
            }
        }
    }

    Ok(true)
}
