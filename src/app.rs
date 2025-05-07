use std::fs;

use eframe::egui::{self, TextEdit, Label, Sense, DragValue};
use time::{Date, OffsetDateTime, format_description};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Entry {
    pub content: String,
    pub weight_kg: f32,
    pub waist_cm: f32,
    pub date: Date,

    #[serde(default)]
    pub edit: bool,
}


#[derive(serde::Serialize, serde::Deserialize)]
pub enum Mode {
    Main,
    Edit,
}


#[derive(serde::Serialize, serde::Deserialize)]
pub struct MyApp {
    pub entries: Vec<Entry>,
    pub curr_date: Date,
    pub mode: Mode,

    pub first_time_edit: bool,
    pub scale_factor: f32,
    pub path_to_file: String,
}

impl MyApp {
    fn default() -> Self {
        MyApp {
            entries: vec![],
            curr_date: OffsetDateTime::now_local().unwrap().date(),
            mode: Mode::Main,

            first_time_edit: false,
            scale_factor: 2.0,
            path_to_file: String::from("diary.json"),
        }
    }
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            if let Some(mut app) = eframe::get_value::<MyApp>(storage, eframe::APP_KEY) {
                app.curr_date = OffsetDateTime::now_local().unwrap().date();
                app
            } else {
                MyApp::default()
            }
        } else {
            MyApp::default()
        }
    }

    pub fn save_to_file(&self) {
        fs::write(&self.path_to_file, &serde_json::to_vec_pretty(&self.entries).expect("DB should be writeable")).expect("DB should be writeable");
    }

    pub fn load_from_file(&mut self) {
        if let Ok(file_contents) = fs::read_to_string(&self.path_to_file){
            self.entries = serde_json::from_str(&file_contents).unwrap();
        }
    }

    pub fn get_entry_by_date(&self, date: Date) -> Option<Entry> {
        if let Some(entry) = self.entries.iter().find(|entry| entry.date == date) {
            return Some(entry.clone());
        } else {
            return None;
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Variables used in all layouts
        //let weight_vec: Vec<f32>;
        //let waist_vec: Vec<f32>;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Section with diary entries
                    match self.mode {
                        Mode::Main => {
                            // Handle zooming
                            if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                                self.scale_factor += 0.2;

                                if self.scale_factor > 3.0 {
                                    self.scale_factor = 3.0;
                                }

                                ctx.set_pixels_per_point(self.scale_factor);
                            }
                            if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                                self.scale_factor -= 0.2;

                                if self.scale_factor < 1.0 {
                                    self.scale_factor = 1.0;
                                }

                                ctx.set_pixels_per_point(self.scale_factor);
                            }

                            // If there is no entry for today, add a prompt for it
                            if let None = self.get_entry_by_date(self.curr_date) {
                                let format = format_description::parse("[day]-[month]-[year]").unwrap();
                                let date_string = self.curr_date.format(&format).unwrap();
                                ui.heading(date_string);
                                if ui.add(Label::new("Add entry for today!").sense(Sense::click())).clicked() {
                                    let new_entry = Entry {
                                        content: String::new(),
                                        weight_kg: 0.0,
                                        waist_cm: 0.0,
                                        date: self.curr_date,
                                        edit: true,
                                    };

                                    self.entries.insert(0, new_entry);

                                    self.mode = Mode::Edit;
                                    self.first_time_edit = true;
                                }

                                ui.add_space(10.0);
                            }

                            for entry in &mut self.entries {
                                let format = format_description::parse("[day]-[month]-[year]").unwrap();
                                let date_string = entry.date.format(&format).unwrap();

                                ui.horizontal(|ui| {
                                    let mut weight_string = String::from("--");

                                    if entry.weight_kg != 0.0 {
                                        weight_string = format!("{:.1}", entry.weight_kg);
                                    }
                                    weight_string.push_str(" kg");

                                    let mut waist_string = String::from("--");
                                    if entry.waist_cm != 0.0 {
                                        waist_string = format!("{:.1}", entry.waist_cm);
                                    }
                                    waist_string.push_str(" cm");

                                    ui.heading(date_string);
                                    ui.label(weight_string);
                                    ui.label(waist_string);
                                });

                                if entry.content.len() > 0 {
                                    if ui.add(Label::new(&entry.content).sense(Sense::click())).clicked() {
                                        entry.edit = true;
                                        self.mode = Mode::Edit;
                                        self.first_time_edit = true;
                                    }
                                    ui.add_space(10.0);
                                }
                            }
                        },

                        Mode::Edit => {
                            for entry in &mut self.entries {
                                let format = format_description::parse("[day]-[month]-[year]").unwrap();
                                let date_string = entry.date.format(&format).unwrap();

                                if entry.edit {
                                    ui.horizontal(|ui| {
                                        ui.heading(date_string);

                                        ui.add(DragValue::new(&mut entry.weight_kg).speed(0.1));
                                        ui.label(" kg");
                                        ui.add(DragValue::new(&mut entry.waist_cm).speed(0.1));
                                        ui.label(" cm");
                                    });

                                    let response = ui.add_sized([ui.available_width(), 1.0], TextEdit::multiline(&mut entry.content));

                                    if self.first_time_edit {
                                        response.request_focus();
                                        self.first_time_edit = false;
                                    }

                                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                                        self.mode = Mode::Main;
                                        entry.edit = false;
                                    }
                                } else if entry.content.len() > 0 {
                                    ui.horizontal(|ui| {
                                        ui.heading(date_string);

                                        let mut weight_string = String::from("--");

                                        if entry.weight_kg != 0.0 {
                                            weight_string = format!("{:.1}", entry.weight_kg);
                                        }
                                        weight_string.push_str(" kg");

                                        let mut waist_string = String::from("--");
                                        if entry.waist_cm != 0.0 {
                                            waist_string = format!("{:.1}", entry.waist_cm);
                                        }
                                        waist_string.push_str(" cm");
                                    });

                                    ui.label(&entry.content);
                                }

                                ui.add_space(10.0);
                            }
                        }
                    }
                });

                    // Section with graphs
            });
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }
}
