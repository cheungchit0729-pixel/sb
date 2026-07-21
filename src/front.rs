use chrono::NaiveDateTime;
use eframe::egui;
use crate::back;
use crate::back::{AppStorage, PurchaseEntry};

#[derive(Default)]
pub(crate) struct App {
    // dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
    show_confirmation_dialog: bool,
    allowed_to_close: bool,

    settings: bool,
    about: bool,
    storage: AppStorage,
    editor: Option<PurchaseEntryEditor>,
    editing_index: Option<usize>,
    new_editor: PurchaseEntryEditor,
}

impl App {
    pub(crate) fn name() -> &'static str {
        "app"
    }
}

#[derive(Default)]
pub struct PurchaseEntryEditor {
    pub date_text: String,
    pub amount_text: String,
    pub merchant: String,
    pub category: String,
    pub notes: String,
}
impl PurchaseEntryEditor {
    pub fn apply_to(&self, e: &mut PurchaseEntry) {
        if let Ok(dt) = NaiveDateTime::parse_from_str(&self.date_text, "%Y-%m-%d %H:%M:%S") {
            e.date = dt;
        }
        if let Ok(v) = self.amount_text.parse::<f64>() {
            e.amount = v;
        }
        e.merchant = self.merchant.clone();
        e.category = self.category.clone();
        e.notes = self.notes.clone();
    }

    fn from(e: &PurchaseEntry) -> Self {
        Self {
            date_text: e.date.format("%Y-%m-%d %H:%M:%S").to_string(),
            amount_text: e.amount.to_string(),
            merchant: e.merchant.clone(),
            category: e.category.clone(),
            notes: e.notes.clone(),
        }
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
                            if self.picked_path == None { return; }
                            self.storage.add_many(back::parse_data(self.picked_path.clone()))
                        }
                    }
                    ui.separator();
                    if ui.button("Save...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().save_file() {
                            self.picked_path = Some(path.display().to_string());
                        }
                    }
                    if ui.button("Save as...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().save_file() {
                            self.picked_path = Some(path.display().to_string());
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
                        //functionality
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

            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    if self.picked_path.is_none() {
                        ui.label("Load a file first!");
                        return;
                    }
                    if self.storage.get_all().len() == 0 {
                        ui.label("Nothing to see here!");
                        return;
                    }

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let mut to_delete: Vec<usize> = Vec::new();
                        let mut clicked_edit: Option<usize> = None;
                        let mut clicked_save: bool = false; // or just bool, depending on your UI

                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for (i, e) in self.storage.get_all().iter().enumerate() {
                                ui.horizontal(|ui| {
                                    ui.label(e.date.to_string());
                                    ui.label(format!("{:.3}", e.amount));
                                    ui.label(&e.merchant);
                                    ui.label(&e.category);
                                    ui.label(&e.notes);

                                    if (i == self.editing_index.expect("no editing index")) {

                                        if ui.button("Save").clicked() {
                                            clicked_save = true;
                                        }
                                    }

                                    if ui.button("EDIT").clicked() {
                                        clicked_edit = Some(i);
                                    }

                                    if ui.button("DELETE").clicked() {
                                        to_delete.push(i);
                                    }
                                });
                                ui.separator();
                            }
                        });

                        // EDIT
                        if let Some(i) = clicked_edit {
                            println!("{:?} {:?}","editing",i); // working here
                            let entry_ref = self.storage.get(Option::from(i));
                            self.editing_index = Some(i);
                            self.editor = Some(PurchaseEntryEditor::from(entry_ref.expect("Missing entry reference")));
                        }

                        // SAVE
                        if clicked_save {
                            if let (Some(i), Some(editor)) = (self.editing_index, self.editor.as_ref()) {
                                if let Some(entry_mut) = self.storage.get_mut(Option::from(i)) {
                                    editor.apply_to(entry_mut);
                                }
                            }
                            clicked_save = false;
                        }

                        // DELETE
                        to_delete.sort_unstable();
                        to_delete.dedup();
                        for i in to_delete.into_iter().rev() {
                            self.storage.remove(i);
                        }
                    });


                    ui.horizontal(|ui| {
                        ui.add_sized([80.0,20.0],
                                     egui::TextEdit::singleline(&mut self.new_editor.date_text).hint_text("Date"));
                        ui.add_sized([80.0,20.0],
                                     egui::TextEdit::singleline(&mut self.new_editor.amount_text).hint_text("Amount"));
                        ui.add_sized([80.0,20.0],
                                     egui::TextEdit::singleline(&mut self.new_editor.merchant).hint_text("Merchant"));
                        ui.add_sized([80.0,20.0],
                                     egui::TextEdit::singleline(&mut self.new_editor.category).hint_text("Category"));
                        ui.add_sized([80.0,20.0],
                                     egui::TextEdit::singleline(&mut self.new_editor.notes).hint_text("Notes"));
                    });

                    if ui.button("ADD").clicked() {
                        // create a new entry from the editor and push it
                        let mut entry = PurchaseEntry {
                            date: NaiveDateTime::default(),
                            amount: 0.0,
                            merchant: String::new(),
                            category: String::new(),
                            notes: String::new(),
                        };

                        self.new_editor.apply_to(&mut entry);
                        self.storage.add(entry);

                        // clear editor for next add
                        self.new_editor = PurchaseEntryEditor {
                            date_text: String::new(),
                            amount_text: String::new(),
                            merchant: String::new(),
                            category: String::new(),
                            notes: String::new(),
                        };
                    }

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
                    ui.label("about");
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
                .show(
                    ctx,
                    |ui| {
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
                    }
                );
        }
    }
}