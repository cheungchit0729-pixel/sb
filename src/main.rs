#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono;
use csv;
use eframe::egui;
use std::io;

#[derive(Debug, serde::Deserialize)]
enum Category {
    Transportation,
    Utilities,
    Food,
    Gifts,
}

#[derive(Debug, serde::Deserialize)]
struct PurchaseEntry {
    date: chrono::NaiveDateTime,
    amount: f64,
    merchant: String,
    description: String,
    category: Category,
    notes: String,
}

#[derive(Default)]
struct App {
    // dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
    loaded_data: Vec<PurchaseEntry>,
    show_confirmation_dialog: bool,
    allowed_to_close: bool,
    settings: bool,
    about: bool,
}

impl App {
    fn name() -> &'static str {
        "app"
    }

    fn parse_data(path: Option<String>) -> Vec<PurchaseEntry> {
        println!("reading data");
        let mut entries: Vec<PurchaseEntry> = Vec::new();
        let mut rdr = csv::Reader::from_reader(io::stdin());
        for result in rdr.deserialize() {
            // The iterator yields Result<StringRecord, Error>, so we check the
            // error here.
            let record: PurchaseEntry = result.expect("REASON");
            entries.push(record);
        }
        entries
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // fixed pixel
        // ctx.set_pixels_per_point(1.25);
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // menu bar
            egui::menu::bar(ui, |ui| {
                // first menu
                ui.menu_button("File", |ui| {
                    if ui.button("Open new...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.picked_path = Some(path.display().to_string());
                            self.loaded_data = Self::parse_data(self.picked_path.clone())
                        }
                    }
                    if ui.button("Open recent...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.picked_path = Some(path.display().to_string());
                            println!("{:?}", self.picked_path);
                        }
                    }
                    ui.separator();
                    if ui.button("Save...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().save_file() {
                            self.picked_path = Some(path.display().to_string());
                            println!("{:?}", self.picked_path);
                        }
                    }
                    if ui.button("Save as...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().save_file() {
                            self.picked_path = Some(path.display().to_string());
                            println!("{:?}", self.picked_path);
                        }
                    }
                    // confirm on quit
                    ui.separator();
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                });
                // Tools
                ui.menu_button("Tools", |ui| {
                    if ui.button("Preferences").clicked() {
                        unimplemented!()
                    }
                    if ui.button("Settings").clicked() {
                        self.settings = true;
                    }
                });
                // Help
                ui.menu_button("Help", |ui| {
                    ui.separator();
                    if ui.button("About this program").clicked() {
                        //funtionality
                    }
                })
            });
        });
        // CentralPanel == Container
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::SidePanel::left("left_panel")
                .resizable(true)
                .default_width(150.0)
                .width_range(80.0..=200.0)
                .show_inside(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Side menu");
                        ui.separator();
                        if ui.button("Click me!").clicked() {
                            // …
                        }
                    });
                    egui::ScrollArea::vertical().show(ui, |_ui| {});
                });
            // This literally creates the button AND checks to see if it was clicked
            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    egui::Grid::new("help").show(ui, |ui| {
                        if let text = (self.picked_path != None) {
                            ui.label("Load a file first!");
                        }
                        egui::ScrollArea::vertical()
                            .max_height(200.0)
                            .show(ui, |ui| {
                                // everything purchasedata related here
                            });

                        ui.end_row();
                    })
                });
            });
        });

        if ctx.input(|i| i.viewport().close_requested()) {
            if self.allowed_to_close {
                // do nothing - we will close
            } else {
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                self.show_confirmation_dialog = true;
            }
        }
        if self.about {
            egui::Window::new("About")
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.label("change theme");
                    egui::widgets::global_dark_light_mode_buttons(ui);
                    ui.separator();
                    ui.label("another setting");
                    egui::widgets::global_dark_light_mode_buttons(ui);
                });
        }
        if self.settings {
            egui::Window::new("Setting")
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("change theme");
                    egui::widgets::global_dark_light_mode_buttons(ui);
                    ui.separator();
                    ui.label("another setting");
                    egui::widgets::global_dark_light_mode_buttons(ui);
                });
        }
        if self.show_confirmation_dialog {
            egui::Window::new("Do you want to quit?")
                .pivot(egui::Align2::CENTER_TOP)
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("No").clicked() {
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = false;
                        }
                        if ui.button("Yes").clicked() {
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = true;
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
        }
    }
}

// main loop
fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((1280.0, 720.0)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        App::name(),
        native_options,
        Box::new(|_| Box::<App>::default()),
    )
}
