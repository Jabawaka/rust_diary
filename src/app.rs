use std::cmp::Ordering;
use std::fs;

use eframe::egui;
use time::{Date, OffsetDateTime, format_description};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Entry {
    pub content: String,
    pub weight_kg: Option<f32>,
    pub waist_cm: Option<f32>,
    pub date: Date,
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

pub struct MyApp {
    pub entries: Vec<Entry>,
    pub curr_date: Date,
    pub curr_screen: CurrScreen,
    pub edit_value: EditValue,
    pub zoom: ZoomLevel,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = MyApp {
            entries: vec![],
            curr_date: OffsetDateTime::now_local().unwrap().date(),
            curr_screen: CurrScreen::Main,
            edit_value: EditValue::Content,
            zoom: ZoomLevel::Day,
        };
        app.load_from_file();

        app
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

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Variables used in all layouts
        let weight_vec: Vec<f32>;
        let waist_vec: Vec<f32>;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Section with diary entries
                    for entry in &mut self.entries {
                        let format = format_description::parse("[day]-[month]-[year]").unwrap();
                        let date_string = entry.date.format(&format).unwrap();

                        if entry.content.len() > 0 {
                            ui.heading(date_string);
                            ui.label(&entry.content);
                            ui.add_space(10.0);
                        }
                    }

                    // Section with graphs
                });
            });
        });
    }
}
