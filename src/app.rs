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
    pub fn new(date: Date) -> Entry {
        Entry {
            content: String::new(),
            weight_kg: None,
            waist_cm: None,
            date: date,
        }
    }

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

pub struct EditString {
    prev: String,
    next: String,
}

impl EditString {
    pub fn new() -> EditString {
        EditString {
            prev: String::new(),
            next: String::new()
        }
    }

    pub fn from(string: String, cursor_pos: usize) -> EditString {
        if cursor_pos == 0 {
            // This means to put the cursor at the end of the string
            EditString {
                prev: string.clone(),
                next: String::new()
            }
        } else {
            if cursor_pos > string.len() {
                EditString {
                    prev: string.clone(),
                    next: String::new()
                }
            } else {
                let (prev, next) = string.split_at(cursor_pos);
                EditString {
                    prev: String::from(prev),
                    next: String::from(next)
                }
            }
        }
    }

    pub fn add_char(&mut self, c: char) {
        self.prev.push(c);
    }

    pub fn remove_char(&mut self) {
        if self.prev.len() > 0 {
            self.prev.pop();
        }
    }

    pub fn next_cursor(&mut self) {
        if self.next.len() > 0 {
            let char = self.next.remove(0);
            self.prev.push(char);
        }
    }

    pub fn prev_cursor(&mut self) {
        if self.prev.len() > 0 {
            let char = self.prev.pop().unwrap();
            self.next.insert(0, char);
        }
    }

    pub fn to_string(&mut self) -> String {
        let mut final_string = String::from(&self.prev);
        final_string.push_str(&self.next);

        final_string
    }
}

pub enum CurrScreen {
    Main,
    Editing,
}

pub enum EditValue {
    Content,
    Weight,
    Waist
}

#[derive(Copy, Clone)]
pub enum ZoomLevel {
    Day,
    Week,
    Month,
}

pub const DAYS_IN_A_WEEK: u16 = 7;

pub struct App {
    pub entries: Vec<Entry>,
    pub edit_content: EditString,
    pub edit_weight: EditString,
    pub edit_waist: EditString,
    pub curr_date: Date,
    pub curr_screen: CurrScreen,
    pub edit_value: EditValue,
    pub zoom: ZoomLevel,
}

impl App {
    pub fn new() -> App {
        App {
            entries: vec![],
            edit_content: EditString::new(),
            edit_weight: EditString::new(),
            edit_waist: EditString::new(),
            curr_date: OffsetDateTime::now_local().unwrap().date(),
            curr_screen: CurrScreen::Main,
            edit_value: EditValue::Content,
            zoom: ZoomLevel::Day,
        }
    }

    pub fn enter_edit_mode(&mut self) {
        self.curr_screen = CurrScreen::Editing;
        self.edit_value = EditValue::Content;

        if let Some(entry) = self.get_entry_by_date(self.curr_date) {
            self.edit_content = EditString::from(entry.content, 0);

            if let Some(weight) = entry.weight_kg {
                self.edit_weight = EditString::from(weight.to_string(), 0);
            } else {
                self.edit_weight = EditString::new();
            }

            if let Some(waist) = entry.waist_cm {
                self.edit_waist = EditString::from(waist.to_string(), 0);
            } else {
                self.edit_waist = EditString::new();
            }
        } else {
            self.edit_content = EditString::new();
            self.edit_weight = EditString::new();
            self.edit_waist = EditString::new();
        }
    }

    pub fn enter_main_mode(&mut self) {
        self.curr_screen = CurrScreen::Main;

        let final_weight_kg;
        if let Ok(weight_kg) = self.edit_weight.to_string().parse::<f32>() {
            final_weight_kg = Some(weight_kg);
        } else {
            final_weight_kg = None;
        }

        let final_waist_cm;
        if let Ok(waist_cm) = self.edit_waist.to_string().parse::<f32>() {
            final_waist_cm = Some(waist_cm);
        } else {
            final_waist_cm = None;
        }

        let entry_to_save = Entry {
            content: self.edit_content.to_string(),
            weight_kg: final_weight_kg,
            waist_cm: final_waist_cm,
            date: self.curr_date
        };

        self.save_entry(entry_to_save);
    }

    pub fn type_char(&mut self, c: char) {
        match self.edit_value {
            EditValue::Content => self.edit_content.add_char(c),
            EditValue::Weight => {
                if c.is_digit(10) || c == '.' {
                    self.edit_weight.add_char(c);
                }
            }
            EditValue::Waist => {
                if c.is_digit(10) || c == '.' {
                    self.edit_waist.add_char(c);
                }
            }
        }
    }

    pub fn remove_char(&mut self) {
        match self.edit_value {
            EditValue::Content => self.edit_content.remove_char(),
            EditValue::Weight  => self.edit_weight.remove_char(),
            EditValue::Waist   => self.edit_waist.remove_char(),
        }
    }

    pub fn cycle_edit_value(&mut self) {
        match self.edit_value {
            EditValue::Content => self.edit_value = EditValue::Weight,
            EditValue::Weight  => self.edit_value = EditValue::Waist,
            EditValue::Waist   => self.edit_value = EditValue::Content,
        }
    }

    pub fn next_cursor(&mut self) {
        match self.edit_value {
            EditValue::Content => self.edit_content.next_cursor(),
            EditValue::Weight  => self.edit_weight.next_cursor(),
            EditValue::Waist   => self.edit_waist.next_cursor(),
        }
    }

    pub fn prev_cursor(&mut self) {
        match self.edit_value {
            EditValue::Content => self.edit_content.prev_cursor(),
            EditValue::Weight  => self.edit_weight.prev_cursor(),
            EditValue::Waist   => self.edit_waist.prev_cursor(),
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
        }

        waists
    }

    pub fn print_entries(&self) {
        for entry in self.entries.iter() {
            entry.print_redux();
        }
    }
}