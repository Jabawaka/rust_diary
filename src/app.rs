use std::cmp::Ordering;
use std::fs;
use time::{Date, OffsetDateTime};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Entry {
    pub content: String,
    pub weight_kg: Option<f32>,
    pub waist_cm: Option<f32>,
    pub date: Date,
}

pub struct EditString {
    pub prev: String,
    pub next: String,
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

    pub fn to_final_string(&self) -> String {
        let mut final_string = String::from(&self.prev);
        final_string.push_str(&self.next);

        final_string
    }

    pub fn to_edit_string(&self) -> String {
        let mut edit_string = String::from(&self.prev);
        edit_string.push('_');

        let mut next_str = String::from(&self.next);
        if next_str.len() > 0 {
            next_str.remove(0);
        }
        edit_string.push_str(&next_str);

        edit_string
    }
}

pub enum CurrScreen {
    Main,
    Editing,
    DiscardChanges,
}

#[derive(PartialEq)]
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

pub const GRAPH_POINTS: u8 = 8;

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

    pub fn save_to_file(&self) {
        let file = String::from("diary.json");
        fs::write(file, &serde_json::to_vec_pretty(&self.entries).expect("DB should be writeable")).expect("DB should be writeable");
    }

    pub fn load_from_file(&mut self) {
        if let Ok(file_contents) = fs::read_to_string("diary.json") {
            self.entries = serde_json::from_str(&file_contents).unwrap();
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

        let content = self.edit_content.to_final_string();

        let final_weight_kg;
        if let Ok(weight_kg) = self.edit_weight.to_final_string().parse::<f32>() {
            final_weight_kg = Some(weight_kg);
        } else {
            final_weight_kg = None;
        }

        let final_waist_cm;
        if let Ok(waist_cm) = self.edit_waist.to_final_string().parse::<f32>() {
            final_waist_cm = Some(waist_cm);
        } else {
            final_waist_cm = None;
        }

        if content.len() > 0 || final_weight_kg != None || final_waist_cm != None {
            let entry_to_save = Entry {
                content: content,
                weight_kg: final_weight_kg,
                waist_cm: final_waist_cm,
                date: self.curr_date
            };

            self.save_entry(entry_to_save);
        }
    }

    pub fn type_char(&mut self, c: char) {
        match self.edit_value {
            EditValue::Content => {
                self.edit_content.add_char(c);
            }
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

    pub fn next_zoom(&mut self) {
        match self.zoom {
            ZoomLevel::Day  => self.zoom = ZoomLevel::Week,
            ZoomLevel::Week => self.zoom = ZoomLevel::Month,
            _ => {}
        }
    }

    pub fn prev_zoom(&mut self) {
        match self.zoom {
            ZoomLevel::Week  => self.zoom = ZoomLevel::Day,
            ZoomLevel::Month => self.zoom = ZoomLevel::Week,
            _ => {}
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
        let mut x_axis = 0.0;

        match zoom_level {
            ZoomLevel::Day => {
                let mut curr_day = date.prev_occurrence(date.weekday());

                while curr_day <= date {
                    if let Some(entry) = self.get_entry_by_date(curr_day) {
                        if let Some(weight_kg) = entry.weight_kg {
                            weights.push((x_axis, weight_kg as f64));
                        }
                    }

                    curr_day = curr_day.next_day().unwrap();
                    x_axis = x_axis + 1.0;
                }
            }
            ZoomLevel::Week => {
                let mut curr_week = GRAPH_POINTS;

                while curr_week > 0 {
                    let mut sum_weight_kg = 0.0;
                    let mut num_points = 0;

                    let mut curr_day;
                    let last_day;
                    if curr_week > 1 {
                        curr_day = date.nth_prev_occurrence(date.weekday(), curr_week).next_day().unwrap();
                        last_day = date.nth_prev_occurrence(date.weekday(), curr_week - 1);
                    } else {
                        curr_day = date.prev_occurrence(date.weekday()).next_day().unwrap();
                        last_day = date;
                    }

                    while curr_day <= last_day {
                        if let Some(entry) = self.get_entry_by_date(curr_day) {
                            if let Some(weight_kg) = entry.weight_kg {
                                sum_weight_kg = sum_weight_kg + weight_kg;
                                num_points = num_points + 1;
                            }
                        }

                        curr_day = curr_day.next_day().unwrap()
                    }

                    if num_points > 0 {
                        weights.push((x_axis, (sum_weight_kg as f64 / (num_points as f64))));
                    } else {
                        weights.push((x_axis, 0.0));
                    }
                    curr_week = curr_week - 1;
                    x_axis = x_axis + 1.0;
                }
            }
            ZoomLevel::Month => {
            }
        }

        weights
    }

    pub fn get_waists(&self, date: Date, zoom_level: ZoomLevel) -> Vec<(f64, f64)> {
        let mut waists = Vec::new();
        let mut x_axis = 0.0;

        match zoom_level {
            ZoomLevel::Day => {
                let mut curr_day = date.prev_occurrence(date.weekday());

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
                let mut curr_week = GRAPH_POINTS;

                while curr_week > 0 {
                    let mut sum_waist_cm = 0.0;
                    let mut num_points = 0;

                    let mut curr_day;
                    let last_day;
                    if curr_week > 1 {
                        curr_day = date.nth_prev_occurrence(date.weekday(), curr_week).next_day().unwrap();
                        last_day = date.nth_prev_occurrence(date.weekday(), curr_week - 1);
                    } else {
                        curr_day = date.prev_occurrence(date.weekday()).next_day().unwrap();
                        last_day = date;
                    }

                    while curr_day <= last_day {
                        if let Some(entry) = self.get_entry_by_date(curr_day) {
                            if let Some(waist_cm) = entry.waist_cm {
                                sum_waist_cm = sum_waist_cm + waist_cm;
                                num_points = num_points + 1;
                            }
                        }

                        curr_day = curr_day.next_day().unwrap()
                    }

                    if num_points > 0 {
                        waists.push((x_axis, (sum_waist_cm as f64 / (num_points as f64))));
                    } else {
                        waists.push((x_axis, 0.0));
                    }
                    curr_week = curr_week - 1;
                    x_axis = x_axis + 1.0;
                }
            }
            ZoomLevel::Month => {
            }
        }

        waists
    }
}