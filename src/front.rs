use crate::back::{AppStorage, PurchaseEntry, new_data, parse_data, write_data};
use chrono::NaiveDateTime;
use eframe::egui;
use elegance::{Accent, Button, Card, Checkbox, TextInput, Theme};
use std::cmp::PartialEq;
use std::vec::Vec;
use egui::InnerResponse;
use uuid::Uuid;


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
#[derive(Default, Eq, PartialEq)]
pub enum RowAction {
    Save,
    Delete,
    #[default]
    None,
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
    edit_id: Option<Uuid>,
    new_editor: PurchaseEntryEditor,
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
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
        Theme::paper().install(ui.ctx()); // apply theme

        egui::Panel::top("tmenu").show(ui, |ui| {
            self.top_menu(ui);
        });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                let left_width = ui.available_width() * 0.15;

                ui.allocate_ui(egui::Vec2::new(left_width, ui.available_height()), |ui| {
                    self.side_panel(ui);
                });

                ui.vertical(|ui| {
                    self.current_tab(ui);
                    }
                )
            });
        });


        if ui.input(|i| i.viewport().close_requested()) {
            self.handle_close_request(ui);
        }

        // Quit confirmation dialog
        if self.show_confirmation_dialog {
            self.confirm_quit_dialog(ui);
        }
    }
}

impl App {
    pub(crate) fn name() -> &'static str {
        "app"
    }
    fn top_menu(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.menu_button("File", |ui| {
                self.file_menu(ui);
            });
            ui.menu_button("Tools", |ui| {
                self.tools_menu(ui);
            });
        });
    }

    fn file_menu(&mut self, ui: &mut egui::Ui) {
        if ui.button("New").clicked() {
            self.picked_path = None;
            self.storage.purge();
            self.storage.add(new_data());
            println!("made new thing")
        }

        ui.separator();

        if ui.button("Open new...").clicked() {
            self.open_file_dialog();
        }

        ui.separator();

        if ui.button("Save").clicked() {
            let _ = write_data(self.picked_path.clone(), self.storage.get_all());
        }

        if ui.button("Save as...").clicked() {
            if let Some(path) = rfd::FileDialog::new().save_file() {
                self.picked_path = Some(path.display().to_string());
                let _ = write_data(self.picked_path.clone(), self.storage.get_all());
            }
        }

        ui.separator();

        if ui.button("Quit").clicked() {
            std::process::exit(0);
        }
    }

    fn tools_menu(&mut self, ui: &mut egui::Ui) {
        if ui.button("Preferences").clicked() {
            unimplemented!()
        }
        if ui.button("Settings").clicked() {
            self.current_tab = Tabs::Settings;
        }
        if ui.button("About").clicked() {
            self.current_tab = Tabs::About; // or unimplemented!() if no window
        }
    }

    fn open_file_dialog(&mut self) {
        self.storage.purge();
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            self.picked_path = Some(path.display().to_string());
            for entry in parse_data(self.picked_path.clone()) {
                self.storage.add(entry);
            }
        }
    }

    fn side_panel(&mut self, ui: &mut egui::Ui) {
        Card::new().heading("Tabs").show(ui, |ui| {
            ui.vertical(|ui| {
                self.tab_button(ui, "Overview", Tabs::Main);
                self.tab_button(ui, "Purchases", Tabs::Data);
                self.tab_button(ui, "Searching", Tabs::Searching);
                self.tab_button(ui, "Sorting", Tabs::Sorting);

                // keep these buttons but you can decide whether you want them visible
                self.tab_button(ui, "Settings", Tabs::Settings);
                self.tab_button(ui, "About", Tabs::About);
            });
        });
    }
    fn tab_button(&mut self, ui: &mut egui::Ui, label: &str, tab: Tabs) {
        if ui.button(label).clicked() {
            self.current_tab = tab;
        }
    }

    fn current_tab(&mut self, ui: &mut egui::Ui) {
        match self.current_tab {
            Tabs::Main => {
                Card::new().heading("Overview").show(ui, |ui| self.main_tab(ui));
            }
            Tabs::Data => {
                Card::new().heading("Purchases").show(ui, |ui| self.data_tab(ui));
            }
            Tabs::Searching => {
                Card::new().heading("Searching").show(ui, |ui| self.searching_tab(ui));
            }
            Tabs::Sorting => {
                Card::new().heading("Sorting").show(ui, |ui| self.sorting_tab(ui));
            }
            Tabs::Settings | Tabs::About => {
                // leave them alone; they are handled by the existing if blocks in ui()
            }
        }
    }


    fn main_tab(&self, ui: &mut egui::Ui) {
        Card::new().heading("eSpenXe").show(ui, |ui| {
            ui.label("Welcome to your personal expense tracker.");
        });
    }

    fn data_tab(&mut self, ui: &mut egui::Ui) {

        if self.storage.get_all().is_empty() {
            ui.label("Nothing to see here!");
            self.new_entry(ui);
            return;
        }

        let entries: Vec<PurchaseEntry> = self.storage.get_all().iter().cloned().collect();

        let mut to_save: Vec<Uuid> = Vec::new();
        let mut to_delete: Vec<Uuid> = Vec::new();

        // Reserve space for the bottom add card, and clamp the list height to the rest.
        let bottom_height = 50.0_f32; // <-- adjust after you run once
        let available = ui.available_height();
        let list_height = (available - bottom_height).max(60.0);

        ui.vertical(|ui| {
            ui.add_space(4.0);

            egui::ScrollArea::vertical()
                .max_height(list_height)
                .show(ui, |ui| {
                    for index in 0..entries.len() {
                        let entry_snapshot = &entries[index];
                        let id = entry_snapshot.id;

                        ui.push_id(id, |ui| {
                            let action = self.purchase_entry_ui(ui, id, entry_snapshot);

                            if let RowAction::Save = action {
                                to_save.push(id);
                            }
                            if let RowAction::Delete = action {
                                to_delete.push(id);
                            }
                        });

                        ui.add_space(6.0);
                    }
                });

            ui.add_space(8.0);

            // Bottom card stays visible.
            self.new_entry(ui);
        });

        for id in to_delete {
            self.storage.remove(id);
        }

        for &index in to_save.iter() {
            if let Some(e_mut) = self.storage.get_mut(Some(index)) {
                self.editor.apply_to(e_mut);
            }
            self.edit_id = None;
        }
    }

    fn sorting_tab(&mut self, ui: &mut egui::Ui) {}

    fn searching_tab(&mut self, ui: &mut egui::Ui) {}

    fn purchase_entry_ui(
        &mut self,
        ui: &mut egui::Ui,
        id: Uuid,
        entry: &PurchaseEntry,
    ) -> RowAction {
        let mut action = RowAction::None;
        if self.edit_id == Some(id) {
            Card::new().show(ui, |ui| {
                ui.horizontal(|ui| {
                    action = self.editing_entry(ui, id);
                });
            });
        } else {
            Card::new().show(ui, |ui| {
                self.entry_label(ui, id, entry);
            });
            action = RowAction::None;
        }
        action
    }

    fn editing_entry(&mut self, ui: &mut egui::Ui, id: Uuid) -> RowAction {
        let mut action = RowAction::None;

        ui.horizontal(|ui| {
            App::render_editing_ui(
                ui,
                &mut self.editor.date_text,
                &mut self.editor.amount_text,
                &mut self.editor.merchant,
                &mut self.editor.category,
                &mut self.editor.notes,
            );

            if ui.button("SAVE").clicked() {
                action = RowAction::Save;
            } else if ui.button("DELETE").clicked() {
                action = RowAction::Delete;
            }
        });

        action
    }


    fn render_editing_ui(
        ui: &mut egui::Ui,
        date_text: &mut String,
        amount_text: &mut String,
        merchant: &mut String,
        category: &mut SpendingCategory,
        notes: &mut String,
    ) {
        ui.horizontal(|ui| {
            ui.add_sized(
                [160.0, 20.0],
                egui::TextEdit::singleline(date_text).hint_text("Date"),
            );
            ui.add_sized(
                [80.0, 20.0],
                egui::TextEdit::singleline(amount_text).hint_text("Amount"),
            );
            ui.add_sized(
                [80.0, 20.0],
                egui::TextEdit::singleline(merchant).hint_text("Merchant"),
            );

            egui::ComboBox::from_label("Category")
                .width(80.0)
                .selected_text(category.as_str())
                .show_ui(ui, |ui| {
                    for c in SpendingCategory::ALL.iter().copied() {
                        ui.selectable_value(category, c, c.as_str());
                    }
                });

            ui.add_sized(
                [80.0, 20.0],
                egui::TextEdit::singleline(notes).hint_text("Notes"),
            );
        });
    }


    fn entry_label(&mut self, ui: &mut egui::Ui, index: Uuid, e: &PurchaseEntry) {
        let text = format!(
            "{}  {:.3}  {}  {}  {}",
            e.date,
            e.amount,
            e.merchant,
            e.category.as_str(),
            e.notes
        );
        if ui
            .add(egui::Label::new(text).sense(egui::Sense::click()))
            .clicked()
        {
            self.edit_id = Some(index);
            self.editor = PurchaseEntryEditor::from(e);
        }
    }

    fn new_entry(&mut self, ui: &mut egui::Ui) {
        egui::Panel::bottom("new_editor")
            .resizable(true)
            .show(ui, |ui| {
                ui.add(egui::Label::new("Add Transaction"));
                ui.horizontal(|ui| {
                    ui.add_sized(
                        [160.0, 20.0],
                        egui::TextEdit::singleline(&mut self.new_editor.date_text)
                            .hint_text("Date"),
                    );
                    ui.add_sized(
                        [80.0, 20.0],
                        egui::TextEdit::singleline(&mut self.new_editor.amount_text)
                            .hint_text("Amount"),
                    );
                    ui.add_sized(
                        [80.0, 20.0],
                        egui::TextEdit::singleline(&mut self.new_editor.merchant)
                            .hint_text("Merchant"),
                    );
                    egui::ComboBox::from_label("Category")
                        .width(80.0)
                        .selected_text(self.new_editor.category.as_str())
                        .show_ui(ui, |ui| {
                            for c in SpendingCategory::ALL.iter().copied() {
                                ui.selectable_value(&mut self.new_editor.category, c, c.as_str());
                            }
                        });
                    ui.add_sized(
                        [80.0, 20.0],
                        egui::TextEdit::singleline(&mut self.new_editor.notes).hint_text("Notes"),
                    );

                    if ui.button("ADD").clicked() {
                        let mut entry = PurchaseEntry {
                            id: Uuid::new_v4(),
                            date: NaiveDateTime::default(),
                            amount: 0.0,
                            merchant: String::new(),
                            category: SpendingCategory::Other,
                            notes: String::new(),
                        };
                        self.new_editor.apply_to(&mut entry);
                        self.storage.add(entry);
                        self.new_editor = PurchaseEntryEditor::new();
                    }
                });
            });
    }

    fn handle_close_request(&mut self, ui: &mut egui::Ui) {
        if self.allowed_to_close {
            // allow close
        } else {
            ui.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.show_confirmation_dialog = true;
        }
    }

    fn about_window(&self, ui: &mut egui::Ui) {
        egui::Window::new("About")
            .resizable(false)
            .collapsible(false)
            .show(ui, |ui| {
                ui.label("about");
            });
    }

    fn settings_window(&self, ui: &mut egui::Ui) {
        egui::Window::new("Setting")
            .resizable(false)
            .show(ui, |ui| {
                ui.label("change theme");
                egui::widgets::global_theme_preference_buttons(ui);
                ui.separator();
                ui.label("another setting");
                egui::widgets::global_theme_preference_buttons(ui);
            });
    }

    fn confirm_quit_dialog(&mut self, ui: &mut egui::Ui) {
        egui::Window::new("Do you want to quit?")
            .pivot(egui::Align2::CENTER_TOP)
            .collapsible(false)
            .resizable(false)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("No").clicked() {
                        self.show_confirmation_dialog = false;
                        self.allowed_to_close = false;
                    }
                    if ui.button("Yes").clicked() {
                        self.show_confirmation_dialog = false;
                        self.allowed_to_close = true;
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
    }
}
