use std::ops::RangeInclusive;

use eframe::egui::{self, TextEdit, Label, Sense, DragValue, RichText};
use egui_plot::{Line, Plot, PlotPoints, GridMark};
use ecolor::Color32;
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
pub struct Task {
    text: String,
    done: bool,
    edit: bool,
    delete: bool,
}

impl Task {
    fn default() -> Self {
        Task {
            text: String::from("New task"),
            done: false,
            edit: false,
            delete: false,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Section {
    title: String,
    tasks: Vec<Task>,
    edit: bool,
    delete: bool,
}

impl Section {
    fn default() -> Self {
        Section {
            title: String::from("New Section"),
            tasks: vec![Task::default()],
            edit: true,
            delete: false,
        }
    }

    fn add_task(&mut self, task: &str, edit: bool) {
        self.tasks.push(Task {text: task.to_string(), done: false, edit, delete: false});
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Mode {
    Main,
    Edit
}


#[derive(serde::Serialize, serde::Deserialize)]
pub struct MyApp {
    pub sections: Vec<Section>,
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
            sections: vec![Section::default()],
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
                app.mode = Mode::Main;
                app
            } else {
                MyApp::default()
            }
        } else {
            MyApp::default()
        }
    }

    pub fn get_entry_by_date(&self, date: Date) -> Option<Entry> {
        if let Some(entry) = self.entries.iter().find(|entry| entry.date == date) {
            return Some(entry.clone());
        } else {
            return None;
        }
    }

    pub fn get_weights(&self) -> PlotPoints {
        let curr_date_julian = self.curr_date.to_julian_day();

        let mut weight_points = vec![];

        for entry in &self.entries {
            if entry.weight_kg != 0.0 {
                let entry_date_offset = entry.date.to_julian_day() - curr_date_julian;
                weight_points.push([entry_date_offset as f64, entry.weight_kg as f64]);
            }
        }

        PlotPoints::new(weight_points)
    }

    pub fn get_waists(&self) -> PlotPoints {
        let curr_date_julian = self.curr_date.to_julian_day();

        let mut waist_points = vec![];

        for entry in &self.entries {
            if entry.waist_cm != 0.0 {
                let entry_date_offset = entry.date.to_julian_day() - curr_date_julian;
                waist_points.push([entry_date_offset as f64, entry.waist_cm as f64]);
            }
        }

        PlotPoints::new(waist_points)
    }

    pub fn add_section(&mut self, title: &str, edit: bool) {
        self.sections.push(Section {title: title.to_string(), tasks: vec![], edit, delete: false});
    }

    pub fn clean_tasks(&mut self) {
        for section in &mut self.sections {
            section.tasks.retain(|t| t.done != true);

            if section.tasks.len() == 0 {
                section.delete = true;
            }
        }

        self.sections.retain(|t| t.delete != true);
    }
}

fn x_axis_dates(grid_mark: GridMark, _: &RangeInclusive<f64>) -> String {
    let curr_date_julian = OffsetDateTime::now_local().unwrap().date().to_julian_day();
    let grid_date_julian = curr_date_julian + grid_mark.value.round() as i32;
    let grid_date = Date::from_julian_day(grid_date_julian).unwrap();
    let format = format_description::parse("[day]/[month]").unwrap();
    let date_string = grid_date.format(&format).unwrap();

    date_string
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check date
        if OffsetDateTime::now_local().unwrap().date() > self.curr_date {
            self.curr_date = OffsetDateTime::now_local().unwrap().date();
        }

        egui::SidePanel::right("ToDo").show(ctx, |ui| {
            // ToDo section
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical(|ui| {
                    match self.mode {
                        // In this mode you can click a Task or a Section to edit it,
                        // the checkboxes to mark tasks as completed and add new Tasks
                        // and Sections
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

                            if ui.input(|i| i.key_pressed(egui::Key::C)) {
                                self.clean_tasks();
                            }

                            for section in &mut self.sections {
                                // Render Section title as clickable, if clicked edit it
                                if ui.add(Label::new(RichText::new(&section.title).heading()).sense(Sense::click())).clicked() {
                                    // Enter edit section mode
                                    section.edit = true;
                                    self.mode = Mode::Edit;
                                }

                                // Render Tasks as clickable, if clicked edit it
                                for task in &mut section.tasks {
                                    ui.horizontal(|ui| {
                                        ui.checkbox(&mut task.done, "");
                                        if ui.add(Label::new(&task.text).sense(Sense::click())).clicked() {
                                            task.edit = true;
                                            self.mode = Mode::Edit;
                                            self.first_time_edit = true;
                                        }
                                    });
                                }

                                // Render an invisible Task used to add a Task
                                let response = ui.add(Label::new("                             "));
                                if response.clicked() {
                                    let empty = String::new();
                                    section.add_task(&empty, true);
                                    self.mode = Mode::Edit;
                                    self.first_time_edit = true;
                                }
                            }
                            ui.separator();

                            // Render an invisible Section used to add a Section
                            let response = ui.add(Label::new(RichText::new("                          ").heading()));
                            if response.clicked() {
                                let empty = String::new();
                                self.add_section(&empty, true);
                                self.mode = Mode::Edit;
                                self.first_time_edit = true;
                            }
                        },

                        Mode::Edit => {
                            for section in &mut self.sections {
                                if section.edit {
                                    ui.horizontal(|ui| {
                                        let response = ui.add(TextEdit::singleline(&mut section.title));

                                        if self.first_time_edit {
                                            response.request_focus();
                                            self.first_time_edit = false;
                                        }

                                        if response.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter) || i.key_pressed(egui::Key::Escape)) {
                                            self.mode = Mode::Main;
                                            section.edit = false;
                                        }

                                        if ui.button("-").clicked() {
                                            self.mode = Mode::Main;
                                            section.tasks.clear();
                                            section.delete = true;
                                        }
                                    });
                                } else {
                                    ui.heading(&section.title);
                                }

                                for task in &mut section.tasks {
                                    ui.horizontal(|ui| {
                                        ui.checkbox(&mut task.done, "");
                                        if task.edit {
                                            // Render edit text box for task
                                            let response = ui.add(TextEdit::singleline(&mut task.text));

                                            if self.first_time_edit {
                                                response.request_focus();
                                                self.first_time_edit = false;
                                            }

                                            if response.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter) || i.key_pressed(egui::Key::Escape)) {
                                                self.mode = Mode::Main;
                                                task.edit = false;
                                            }

                                            if ui.button("-").clicked() {
                                                self.mode = Mode::Main;
                                                task.delete = true;
                                            }
                                        } else {
                                            // Render normally
                                            ui.label(&task.text);
                                        }
                                    });
                                }

                                ui.add_space(12.0);

                                section.tasks.retain(|t| t.delete != true);
                            }
                            ui.separator();
                        },
                    }
                });
            });
        });

        // Diary section
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {

                // Section with graphs
                ui.horizontal(|ui| {
                    let weight_points = self.get_weights();
                    let waist_points = self.get_waists();

                    let weight_line = Line::new("Weight", weight_points)
                        .width(1.5)
                        .color(Color32::CYAN);
                    let waist_line = Line::new("test", waist_points)
                        .width(1.5)
                        .color(Color32::CYAN);

                    let half_ui = ui.available_width() / 2.0 - 20.0;

                    Plot::new("weight").view_aspect(1.6)
                        .width(half_ui)
                        .allow_boxed_zoom(false)
                        .allow_double_click_reset(false)
                        .allow_drag(false)
                        .allow_scroll(false)
                        .allow_zoom(false)
                        .show_x(false)
                        .show_y(false)
                        .default_y_bounds(70.0, 90.0)
                        .show_background(false)
                        .x_axis_formatter(x_axis_dates)
                        .y_axis_label("Weight [kg]")
                        .show(ui, |plot_ui| plot_ui.line(weight_line));
                    Plot::new("waist").view_aspect(1.6)
                        .width(half_ui)
                        .allow_boxed_zoom(false)
                        .allow_double_click_reset(false)
                        .allow_drag(false)
                        .allow_scroll(false)
                        .allow_zoom(false)
                        .show_x(false)
                        .show_y(false)
                        .default_y_bounds(70.0, 90.0)
                        .show_background(false)
                        .x_axis_formatter(x_axis_dates)
                        .y_axis_label("Waist [cm]")
                        .show(ui, |plot_ui| plot_ui.line(waist_line));
                });

                // Section with diary entries
                egui::ScrollArea::vertical().show(ui, |ui| {
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

                    // Rest of entries
                    match self.mode {
                        Mode::Main => {
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

                                    if ui.add(Label::new(RichText::new(date_string).heading()).sense(Sense::click())).clicked() {
                                        entry.edit = true;
                                        self.mode = Mode::Edit;
                                        self.first_time_edit = true;
                                    }
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

                                    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                                        self.mode = Mode::Main;
                                        entry.edit = false;
                                    }
                                } else if entry.content.len() > 0 || entry.weight_kg > 0.0 || entry.waist_cm > 0.0 {
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

                                        ui.label(weight_string);
                                        ui.label(waist_string);
                                    });

                                    if entry.content.len() > 0 {
                                        ui.label(&entry.content);
                                    }
                                }

                                ui.add_space(10.0);
                            }

                            self.entries.retain(|t| {t.edit == true || t.content.len() > 0 || t.weight_kg > 0.0 || t.waist_cm > 0.0});
                        },
                    }
                });
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
