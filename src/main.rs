use std::{error::Error, io};

mod app;

use crate::app::MyApp;

fn main() {
    let native_options = eframe::NativeOptions::default();

    let _ = eframe::run_native("Diary",  native_options, Box::new(|cc| Ok(Box::new(MyApp::new(cc)))));
}
