/*
    Brute-Force Prefetch Hashes
    Author: Harel Segev
    04/28/2022
 */

#![windows_subsystem = "windows"]

mod prefetch;
mod bodyfile;
mod errors;

type Result<T> = std::result::Result<T, PrefetchHashCrackerError>;

use std::fs::File;
use std::path::PathBuf;
use rfd;

use eframe::{egui::CentralPanel, epi::App, NativeOptions, run_native};
use eframe::egui::{Context, TextEdit, Vec2, Visuals, ComboBox};
use eframe::epi::{Frame, Storage};

use prefetch::{PfHashFunction, DevicePaths, from_base16};
use errors::PrefetchHashCrackerError;
use bodyfile::BodyfileReader;


struct PrefetchHashCracker {
    executable: String,
    hash: String,

    bodyfile: PathBuf,
    mount_point: String,

    hash_functions: Vec<PfHashFunction>,
    selected_function: usize,

    result: String
}

impl PrefetchHashCracker {
    fn new() -> Self {
        Self {
            executable: String::new(),
            hash: String::new(),

            bodyfile: PathBuf::new(),
            mount_point: String::new(),

            hash_functions: vec![
                PfHashFunction::scca_vista(),
                PfHashFunction::scca_xp()
            ],

            selected_function: 0,
            result: String::new()
        }
    }

    fn render_app(&mut self, ui: &mut eframe::egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Prefetch Hash Cracker");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.add(TextEdit::singleline(&mut self.executable)
                        .hint_text("NOTEPAD.EXE")
                    );

                    self.executable = self.executable.to_uppercase();
                    ui.label("Executable name");
                });

                ui.vertical(|ui| {
                    ui.label("-");
                });

                ui.vertical(|ui| {
                    ui.add(TextEdit::singleline(&mut self.hash)
                        .hint_text("3D2AFDB4")
                        .desired_width(120.0)
                    );

                    self.hash =  self.hash.to_uppercase();
                    ui.label("Prefetch hash");
                });

                ui.vertical(|ui| {
                    ui.label(".pf");
                });

                ui.add_space(30.0);
                ui.vertical(|ui| {
                    ComboBox::from_label( "Hash function").show_index(
                        ui,
                        &mut self.selected_function,
                        self.hash_functions.len(),
                        |i| self.hash_functions[i].to_string()
                    );
                });
            });

            ui.add_space(15.0);
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.add(TextEdit::singleline(
                        &mut self.bodyfile.display().to_string()
                    )
                        .interactive(false)
                        .desired_width(385.0)
                    );

                    ui.label("Bodyfile (TSK 3.0+)");
                });

                ui.vertical(|ui| {
                    if ui.button("Browse…").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.bodyfile = path;
                        }
                    }
                });

                ui.add_space(30.0);
                ui.vertical(|ui| {
                    ui.add(TextEdit::singleline(&mut self.mount_point)
                        .hint_text("C:")
                        .desired_width(120.0)
                    );

                    ui.label("Mount point");
                });
            });

            ui.add_space(35.0);
            ui.vertical_centered(|ui| {
                if ui.button("Crack").clicked() {
                    self.result = self.crack()
                        .unwrap_or_else(|error| Some(error.to_string()))
                        .unwrap_or("failed to crack the prefetch hash".to_owned());
                }

                ui.add_space(5.0);
                ui.add(TextEdit::singleline(&mut self.result)
                    .desired_width(620.0)
                );
            });
        });
    }
}

impl PrefetchHashCracker {
    fn crack(&mut self) -> Result<Option<String>> {
        let bodyfile = File::open(&self.bodyfile)?;
        let reader = BodyfileReader::new(bodyfile, &self.mount_point);

        let hash_function = &self.hash_functions[self.selected_function];
        let hash = from_base16(&self.hash)?;

        for folder in reader {
            let folder = folder?;

            for guess in DevicePaths::new(&folder, &self.executable) {
                if hash_function.hash(&guess) == hash {
                    return Ok(Some(guess));
                }
            }
        }

        Ok(None)
    }
}

impl App for PrefetchHashCracker {
    fn update(&mut self, _ctx: &Context, _frame: &Frame) {
        CentralPanel::default().show(_ctx, |mut ui| {
            self.render_app(&mut ui);
        });
    }

    fn setup(&mut self, _ctx: &Context, _frame: &Frame, _storage: Option<&dyn Storage>) {
        _ctx.set_visuals(Visuals::dark());
    }

    fn name(&self) -> &str {
        "Prefetch Hash Cracker"
    }
}

fn main() {
    let app = PrefetchHashCracker::new();
    let mut window_options = NativeOptions::default();
    window_options.initial_window_size = Some(Vec2 {x: 715.0, y: 240.0});

    run_native(Box::new(app), window_options);
}
