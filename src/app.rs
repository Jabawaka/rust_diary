use std::io;
use std::cmp::Ordering;
use time::{Date, OffsetDateTime};

#[derive(Clone)]
pub struct Entry {
    pub content: String,
    pub weight_kg: f32,
    pub waist_cm: f32,
    pub date: Date,
}

impl Entry {
    #[cfg(debug_assertions)]
    pub fn print(&self) {
        println!(" ---- {} ----", self.date);
        println!("  {} kg, {} cm", self.weight_kg, self.waist_cm);
        println!("  {}", self.content);
    }

    #[cfg(debug_assertions)]
    pub fn print_redux(&self) {
        if self.content.len() < 20 {
            println!(" -- {} -- {} kg, {} cm -- {}", self.date, self.weight_kg, self.waist_cm, &self.content[0..self.content.len()]);
        } else {
            println!(" -- {} -- {} kg, {} cm -- {}", self.date, self.weight_kg, self.waist_cm, &self.content[0..20]);
        }
    }
}

pub struct App {
    pub entries: Vec<Entry>,
}

impl App {
    pub fn new() -> App {
        App {
            entries: vec![],
        }
    }

    pub fn get_entry_by_date(&self, date: Date) -> Option<Entry> {
        if let Some(entry) = self.entries.iter().find(|entry| entry.date == date) {
            return Some(entry.clone());
        } else {
            return None;
        }
    }

    pub fn save_entry(&mut self, entry_to_save: Entry) {
        // Loop to find if the entry already exists
        let mut index = 0;

        if self.entries.len() > 0 {
            while index < self.entries.len() {
                if self.entries[index].date == entry_to_save.date {
                    // Modify the already existing entry and exit the function
                    self.entries[index] = entry_to_save.clone();

                    return;
                } else if self.entries[index].date.cmp(&entry_to_save.date) == Ordering::Greater {
                    // Insert the value at the index and exit the function
                    self.entries.insert(index, entry_to_save);

                    return;
                }

                index = index + 1;
            }
        }

        // If the loop returns, the entry shall be added at the end
        self.entries.push(entry_to_save);
    }

    pub fn print_entries(&self) {
        for entry in self.entries.iter() {
            entry.print_redux();
        }
    }
}