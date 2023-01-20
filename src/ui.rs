use eframe::{egui, App};
use egui::Slider;

use crate::browser;

#[derive(PartialEq, Debug)]
pub enum PaperMode {
    Default,
    MarkingScheme,
    PastPaper,
    GradeThreshold,
}

#[derive(PartialEq, Debug)]
pub enum PaperSeating {
    Default,
    February,
    June,
    November,
}

#[derive(PartialEq, Debug)]
pub struct MyApp {
    pub subject: String,
    pub year: i32,
    pub mode: PaperMode,
    pub seating: PaperSeating,
    pub paper: i32,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp {
            subject: "Balls".to_string(),
            year: 69,
            mode: PaperMode::Default,
            seating: PaperSeating::Default,
            paper: 0,
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Subject:");
                ui.text_edit_singleline(&mut self.subject)
            });

            ui.horizontal(|ui| {
                ui.label("Year:");
                ui.add(Slider::new(&mut self.year, 2000..=2020))
                    .on_hover_text("Year:");
            });

            ui.horizontal(|ui| {
                ui.label("Mode:");
                ui.radio_value(&mut self.mode, PaperMode::PastPaper, "QP");
                ui.radio_value(&mut self.mode, PaperMode::MarkingScheme, "MS");
                ui.radio_value(&mut self.mode, PaperMode::GradeThreshold, "GT");
            });

            ui.horizontal(|ui| {
                ui.label("Seating:");
                ui.radio_value(&mut self.seating, PaperSeating::February, "February");
                ui.radio_value(&mut self.seating, PaperSeating::June, "June");
                ui.radio_value(&mut self.seating, PaperSeating::November, "November");
            });

            ui.horizontal(|ui| {
                ui.label("Paper:");
                ui.radio_value(&mut self.paper, 1, "1");
                ui.radio_value(&mut self.paper, 2, "2");
                ui.radio_value(&mut self.paper, 3, "3");
                ui.radio_value(&mut self.paper, 4, "4");
            });

            ui.separator();

            if ui.button("Confirm").clicked() {
                let url = browser::contruct_url(&self);
                if url.is_some() {
                    if !browser::open_url(url.unwrap()) {
                        println!("Error opening url.");
                    }
                } else {
                    println!("Error processing url.")
                }
            }
        });
    }
}

pub fn start_ui() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2 { x: 320., y: 240.0 }),
        ..Default::default()
    };

    eframe::run_native("Balls", options, Box::new(|_cc| Box::new(MyApp::default())));
}
