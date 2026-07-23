use std::cmp::PartialEq;
use chrono::NaiveDateTime;
use eframe::egui;
use crate::back;
use crate::back::{AppStorage, PurchaseEntry};

#[derive(Default, Eq, PartialEq)]
pub enum Tabs {
    #[default]
    Main,
    Data,
    Sorting,
    Searching,
    Settings,
    About,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SpendingCategory {
    Groceries,
    Rent,
    Utilities,
    Dining,
    Transport,
    Entertainment,
    Health,
    Education,
    Shopping,
    #[default]
    Other,
}
impl SpendingCategory {
    pub const ALL: [SpendingCategory; 10 + 0] = [
        SpendingCategory::Groceries,
        SpendingCategory::Rent,
        SpendingCategory::Utilities,
        SpendingCategory::Dining,
        SpendingCategory::Transport,
        SpendingCategory::Entertainment,
        SpendingCategory::Health,
        SpendingCategory::Education,
        SpendingCategory::Shopping,
        SpendingCategory::Other,
    ];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Groceries => "Groceries",
            Self::Rent => "Rent",
            Self::Utilities => "Utilities",
            Self::Dining => "Dining",
            Self::Transport => "Transport",
            Self::Entertainment => "Entertainment",
            Self::Health => "Health",
            Self::Education => "Education",
            Self::Shopping => "Shopping",
            Self::Other => "Other",
        }
    }
}

#[derive(Default)]
pub(crate) struct App {
    // dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
    show_confirmation_dialog: bool,
    allowed_to_close: bool,
    current_tab: Tabs,

    storage: AppStorage,
    editor: PurchaseEntryEditor,
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
    pub category: SpendingCategory,
    pub notes: String,
}

impl PurchaseEntryEditor {
    fn new() -> PurchaseEntryEditor {
        Self {
            date_text: String::new(),
            amount_text: String::new(),
            merchant: String::new(),
            category: SpendingCategory::Other,
            notes: String::new(),
        }
    }
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

    fn from(e: Option<&PurchaseEntry>) -> Self {
        let e2 = e.unwrap();
        Self {
            date_text: e2.date.format("%Y-%m-%d %H:%M:%S").to_string(),
            amount_text: e2.amount.to_string(),
            merchant: e2.merchant.clone(),
            category: e2.category.clone(),
            notes: e2.notes.clone(),
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
                    if ui.button("New").clicked() {
                        self.storage.purge();
                        self.storage.add_many(back::new_data());
                    }
                    ui.separator();
                    if ui.button("Open new...").clicked() {
                        self.storage.purge();
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.picked_path = Some(path.display().to_string());
                            if self.picked_path == None { return; }
                            self.storage.add_many(back::parse_data(self.picked_path.clone()))
                        }
                    }
                    ui.separator();
                    if ui.button("Save...").clicked() {
                        back::write_data(self.picked_path.clone(),self.storage.get_all()).expect("Can't write data properly")
                    }
                    if ui.button("Save as...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().save_file() {
                            self.picked_path = Some(path.display().to_string());
                            back::write_data(self.picked_path.clone(),self.storage.get_all()).expect("Can't write data properly")
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
                        self.current_tab = Tabs::Settings;
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
                .width_range(80.0..=250.0)
                .show_inside(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Tabs");
                        ui.separator();

                        if ui.button("Overview").clicked() {
                            self.current_tab = Tabs::Main;
                        }

                        if ui.button("Purchases").clicked() {
                            self.current_tab = Tabs::Data;
                        }

                        if ui.button("Searching").clicked() {
                            self.current_tab = Tabs::Searching;
                        }

                        if ui.button("Sorting").clicked() {
                            self.current_tab = Tabs::Sorting;
                        }
                    });
                    egui::ScrollArea::vertical().show(ui, |_ui| {});
                });

            /*(if self.current_tab == Tabs::Main {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label("eSpen Personal eXpense tracker");
                })
            }*/

           if self.current_tab == Tabs::Data {
               egui::CentralPanel::default().show_inside(ui, |ui| {
                   egui::TopBottomPanel::top("main_data")
                       .resizable(true)
                       .default_height(350.0)
                       .height_range(50.0..=350.0)
                       .show_inside(ui, |ui| {
                           if self.picked_path.is_none() {
                               ui.add_sized([350.0, 350.0], egui::Label::new("Load a file first!"));
                               return;
                           }
                           if self.storage.get_all().len() == 0 {
                               ui.label("Nothing to see here!");
                               return;
                           }

                           egui::ScrollArea::vertical().show(
                               ui,
                               |ui| {
                                   let mut to_delete: Vec<usize> = Vec::new();
                                   let mut clicked_save: bool = false;

                                   egui::ScrollArea::vertical().show(ui, |ui| {
                                       for (i, e) in self.storage.get_all().iter().enumerate() {
                                           ui.horizontal(|ui| {
                                               if self.editing_index == Some(i) {
                                                   ui.add_sized([160.0, 20.0], egui::TextEdit::singleline(&mut self.editor.date_text).hint_text("Date"));
                                                   ui.add_sized([80.0, 20.0], egui::TextEdit::singleline(&mut self.editor.amount_text).hint_text("Amount"));
                                                   ui.add_sized([80.0, 20.0], egui::TextEdit::singleline(&mut self.editor.merchant).hint_text("Merchant"));
                                                   egui::ComboBox::from_label("Category").width(80.0)
                                                       .selected_text(self.editor.category.as_str())
                                                       .show_ui(ui, |ui| {
                                                           for c in SpendingCategory::ALL.iter().copied() {
                                                               ui.selectable_value(&mut self.editor.category, c, c.as_str());
                                                           }
                                                       });
                                                   ui.add_sized([80.0, 20.0], egui::TextEdit::singleline(&mut self.editor.notes).hint_text("Notes"));

                                                   if ui.button("Save").clicked() {
                                                       clicked_save = true;
                                                   }

                                                   if ui.button("Delete").clicked() {
                                                       to_delete.push(i);
                                                   }
                                               } else {
                                                   let text = format!(
                                                       "{}  {:.3}  {}  {}  {}",
                                                       e.date, e.amount, e.merchant, e.category.as_str(), e.notes
                                                   );

                                                   if ui.add(
                                                       egui::Label::new(text).sense(egui::Sense::click()))
                                                       .clicked() {
                                                       self.editing_index = Some(i);
                                                       self.editor = PurchaseEntryEditor::from(self.storage.get(Some(i)));
                                                   }
                                               }
                                           });
                                           ui.separator();
                                       }
                                   });

                                   // SAVE
                                   if clicked_save {
                                       if let Some(i) = self.editing_index {
                                           if let Some(entry_mut) = self.storage.get_mut(Some(i)) {
                                               self.editor.apply_to(entry_mut);
                                           }
                                       }
                                       self.editing_index = None;
                                   }

                                   // DELETE
                                   to_delete.sort_unstable();
                                   to_delete.dedup();
                                   for i in to_delete.into_iter().rev() {
                                       self.storage.remove(i);
                                   }
                               }
                           );
                       });

                   egui::TopBottomPanel::bottom("new_editor")
                       .resizable(true)
                       .default_height(350.0)
                       .height_range(50.0..=350.0)
                       .show(ctx, |ui| {

                           ui.horizontal(|ui| {
                               ui.add_sized([160.0, 20.0],
                                            egui::TextEdit::singleline(&mut self.new_editor.date_text).hint_text("Date"));
                               ui.add_sized([80.0, 20.0],
                                            egui::TextEdit::singleline(&mut self.new_editor.amount_text).hint_text("Amount"));
                               ui.add_sized([80.0, 20.0],
                                            egui::TextEdit::singleline(&mut self.new_editor.merchant).hint_text("Merchant"));
                               egui::ComboBox::from_label("Category").width(80.0)
                                   .selected_text(self.editor.category.as_str())
                                   .show_ui(ui, |ui| {
                                       for c in SpendingCategory::ALL.iter().copied() {
                                           ui.selectable_value(&mut self.new_editor.category, c, c.as_str());
                                       }
                                   });
                               ui.add_sized([80.0, 20.0],
                                            egui::TextEdit::singleline(&mut self.new_editor.notes).hint_text("Notes"));

                               if ui.button("ADD").clicked() {
                                   // create a new entry from the editor and push it
                                   let mut entry = PurchaseEntry {
                                       date: NaiveDateTime::default(),
                                       amount: 0.0,
                                       merchant: String::new(),
                                       category: SpendingCategory::Other,
                                       notes: String::new(),
                                   };

                                   self.new_editor.apply_to(&mut entry);
                                   self.storage.add(entry);

                                   // clear editor for next add
                                   self.new_editor = PurchaseEntryEditor::new()
                               }
                           })
                       });


               });
           }
        });

        if ctx.input(|i| i.viewport().close_requested()) {
            if self.allowed_to_close {
                // do nothing - we will close
            } else {
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                self.show_confirmation_dialog = true;
            }
        }
        if self.current_tab == Tabs::About {
            egui::Window::new("About")
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.label("about");
                });
        }

        if self.current_tab == Tabs::Settings {
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