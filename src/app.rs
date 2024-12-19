use std::io;
use std::cmp::Ordering;
use time::{Date, OffsetDateTime};

#[derive(Clone)]
pub struct Entry {
    pub content: String,
    pub weight_kg: Option<f32>,
    pub waist_cm: Option<f32>,
    pub date: Date,
}

impl Entry {
    #[cfg(debug_assertions)]
    pub fn print(&self) {
        println!(" ---- {} ----", self.date);

        let weight_str;
        if let Some(weight_kg) = self.weight_kg {
            weight_str = weight_kg.to_string();
        } else {
            weight_str = String::from("--");
        }

        let waist_str;
        if let Some(waist_cm) = self.waist_cm {
            waist_str = waist_cm.to_string();
        } else {
            waist_str = String::from("--")
        }

        println!("  {} kg, {} cm", weight_str, waist_str);
        println!("  {}", self.content);
    }

    #[cfg(debug_assertions)]
    pub fn print_redux(&self) {
        let content_str;
        if self.content.len() < 20 {
            content_str = self.content.as_str();
        } else {
            content_str = &self.content[0..20];
        }

        let weight_str;
        if let Some(weight_kg) = self.weight_kg {
            weight_str = weight_kg.to_string();
        } else {
            weight_str = String::from("--");
        }

        let waist_str;
        if let Some(waist_cm) = self.waist_cm {
            waist_str = waist_cm.to_string();
        } else {
            waist_str = String::from("--")
        }

        println!(" -- {} -- {} kg, {} cm -- {}", self.date, weight_str, waist_str, content_str);
    }
}

pub enum CurrScreen {
    Main,
    Editing,
}

pub enum ZoomLevel {
    Day,
    Week,
    Month,
    Year,
}

pub const DAYS_IN_A_WEEK: u16 = 7;

pub struct App {
    pub entries: Vec<Entry>,
    pub curr_screen: CurrScreen,
    pub curr_date: Date,
}

impl App {
    pub fn new() -> App {
        App {
            entries: vec![],
            curr_screen: CurrScreen::Main,
            curr_date: OffsetDateTime::now_local().unwrap().date(),
        }
    }

    pub fn get_entry_by_date(&self, date: Date) -> Option<Entry> {
        if let Some(entry) = self.entries.iter().find(|entry| entry.date == date) {
            return Some(entry.clone());
        } else {
            return None;
        }
    }

    pub fn get_weights(&self, date: Date, zoom_level: ZoomLevel) -> Vec<(f64, f64)> {
        let mut weights = Vec::new();

        match zoom_level {
            ZoomLevel::Day => {
                let mut curr_day = date.prev_occurrence(date.weekday());
                let mut x_axis = 0.0;

                while curr_day <= date {
                    if let Some(entry) = self.get_entry_by_date(curr_day) {
                        if let Some(weight) = entry.weight_kg {
                            weights.push((x_axis, weight as f64));
                        }
                    }

                    curr_day = curr_day.next_day().unwrap();
                    x_axis = x_axis + 1.0;
                }
            }
            ZoomLevel::Week => {
            }
            ZoomLevel::Month => {
            }
            ZoomLevel::Year => {
            }
        }

        weights
    }

    pub fn get_waists(&self, date: Date, zoom_level: ZoomLevel) -> Vec<(f64, f64)> {
        let mut waists = Vec::new();

        match zoom_level {
            ZoomLevel::Day => {
                let mut curr_day = date.prev_occurrence(date.weekday());
                let mut x_axis = 0.0;

                while curr_day <= date {
                    if let Some(entry) = self.get_entry_by_date(curr_day) {
                        if let Some(waist) = entry.waist_cm {
                            waists.push((x_axis, waist as f64));
                        }
                    }

                    curr_day = curr_day.next_day().unwrap();
                    x_axis = x_axis + 1.0;
                }
            }
            ZoomLevel::Week => {
            }
            ZoomLevel::Month => {
            }
            ZoomLevel::Year => {
            }
        }

        waists
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