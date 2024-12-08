use std::{error::Error, io};
use time::{Date, Month};

use ratatui::{};


mod app;
mod ui;

use app::{App, Entry};

fn main() {
    let mut app = App::new();

    let today = Date::from_calendar_date(2024, Month::December, 6).unwrap();
    let yesterday = today.previous_day().unwrap();

    if let Some(entry) = app.get_entry_by_date(today) {
        println!("We shouldn't get here, the date does not exist")
    } else {
        let entry = Entry {
            content: String::from("This is a test entry and therefore the content is not meaningful"),
            weight_kg: 90.0,
            waist_cm: 86.0,
            date: today
        };

        app.save_entry(entry);
    }

    if let Some(mut entry) = app.get_entry_by_date(today) {
        entry.weight_kg = 72.0;
        app.save_entry(entry);
    }

    if let Some(entry) = app.get_entry_by_date(yesterday) {
        println!("We shouldn't get here, the date does not exist")
    } else {
        let entry = Entry {
            content: String::from("Test"),
            weight_kg: 89.0,
            waist_cm: 86.0,
            date: yesterday
        };

        app.save_entry(entry);
    }

    app.print_entries();
}
