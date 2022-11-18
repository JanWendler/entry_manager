use std::vec;
use eframe::epaint::Color32;

use egui::{Separator, Rgba, RichText};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    #[serde(skip)]
    generating_entry: bool,

    entries: Vec<Entry>,
    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
    #[serde(skip)]
    temp_entry: Entry,
    #[serde(skip)]
    pined_entry: Option<usize>,

    title_color: Color,
    name_color: Color,
    #[serde(skip)]
    setting_preferences: bool,
    #[serde(skip)]
    search: String,

}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            entries: vec![],
            generating_entry: false,
            temp_entry: Entry::new(),
            pined_entry: None,
            title_color: Color::new(),
            setting_preferences: false,
            search: String::new(),
            name_color: Color::new(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn render_entry(entry: &Entry, ui: &mut eframe::egui::Ui, title_color: Rgba, name_color: Rgba) {
        ui.heading(RichText::new(&entry.name).color(title_color));
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Owner:");
                    ui.colored_label(name_color, &entry.owner);
                });
                ui.horizontal(|ui| {
                    ui.label("Date:");
                    ui.label(&entry.date);
                });
                ui.horizontal(|ui| {
                    ui.label("Location:");
                    ui.label(&entry.location);
                });
            });
            ui.add_space(50.0);
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Tags:");
                    ui.label("&entry.tags.");
                });
                ui.horizontal(|ui| {
                    ui.label("Status:");
                    ui.label(&entry.status);
                });
            });
        });
    }

    fn render_color_list(ui: &mut eframe::egui::Ui, color: &mut Color, label: &str) {
        egui::ComboBox::from_id_source(label)
            .selected_text(&color.name)
            .show_ui(ui, |ui| {
                ui.selectable_value(color, Color {
                    color: Rgba::BLACK,
                    name: "Black".to_string(),
                }, "Black");
                ui.selectable_value(color, Color {
                    color: Rgba::BLUE,
                    name: "Blue".to_string(),
                }, "Blue");
                ui.selectable_value(color, Color {
                    color: Rgba::RED,
                    name: "Red".to_string(),
                }, "Red");
                ui.selectable_value(color, Color {
                    color: Rgba::GREEN,
                    name: "Green".to_string(),
                }, "Green");
                ui.selectable_value(color, Color {
                    color: Rgba::WHITE,
                    name: "White".to_string(),
                }, "White");
                ui.selectable_value(color, Color {
                    color: Rgba::from_rgb(0.0, 1.0, 1.0),
                    name: "Cyan".to_string(),
                }, "Cyan");
                ui.selectable_value(color, Color {
                    color: Rgba::from_rgb(1.0, 1.0, 0.0),
                    name: "Yellow".to_string(),
                }, "Yellow");
                ui.selectable_value(color, Color {
                    color: Rgba::from_rgb(1.0, 0.0, 1.0),
                    name: "Magenta".to_string(),
                }, "Magenta");
            });
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            label,
            value,
            entries,
            generating_entry,
            temp_entry,
            pined_entry,
            title_color,
            setting_preferences,
            search,
            name_color,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New entry").clicked() {
                        *generating_entry = true;
                    }
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("Delete All Entries").clicked() {
                        entries.clear();
                        *pined_entry = None;
                    }
                });
                ui.menu_button("View", |ui| {
                    if ui.button("Preference").clicked() {
                        *setting_preferences = true;
                    }
                });
                ui.menu_button("Help", |ui| {
                    ui.hyperlink_to("Docs", "https://docs.rs/egui/latest/egui/")
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::LEFT), |ui| {
                    ui.text_edit_singleline(search);
                    ui.label("Search:");
                })
            });
        });
        if *generating_entry {
            egui::Window::new("Create a new entry")
                .collapsible(false)
                .resizable(true)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                            ui.text_edit_singleline(&mut temp_entry.name);
                            ui.label("Name: ");
                        });
                    });

                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                            ui.text_edit_singleline(&mut temp_entry.owner);
                            ui.label("Owner: ");
                        });
                    });

                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                            ui.text_edit_singleline(&mut temp_entry.date);
                            ui.label("Date: ");
                        });
                    });
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                            ui.text_edit_singleline(&mut temp_entry.location);
                            ui.label("Location: ");
                        });
                    });
                    let mut tags: &str = "";
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                            ui.text_edit_singleline(&mut tags);
                            ui.label("Tags: ");
                        });
                    });
                    temp_entry.tags.push(Tag::from_str(tags));
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                            ui.text_edit_singleline(&mut temp_entry.status);
                            ui.label("Status: ");
                        });
                    });


                    ui.horizontal(|ui| {
                        if ui.button("Done").clicked() {
                            entries.push(Entry::from_str(
                                &temp_entry.name,
                                &temp_entry.owner,
                                &temp_entry.date,
                                &temp_entry.location,
                                &temp_entry.tags,
                                &temp_entry.status,
                            ));
                            temp_entry.clear();
                            *generating_entry = false;
                        }
                    });
                });
        }
        if *setting_preferences {
            egui::Window::new("Preferences")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Title color: ");
                        TemplateApp::render_color_list(ui, title_color, "Title color:");
                    });
                    ui.horizontal(|ui| {
                        ui.label("Name color: ");
                        TemplateApp::render_color_list(ui, name_color, "Name color:");
                    });
                    if ui.button("Done").clicked() {
                        *setting_preferences = false;
                    }
                });
        }
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            match pined_entry {
                Some(id) => {
                    TemplateApp::render_entry(entries.get(*id).unwrap(), ui, title_color.color, name_color.color);
                    if ui.button("Unpin").clicked() {
                        *pined_entry = None;
                    }
                }
                None => {
                    ui.heading("eframe template");
                    ui.hyperlink("https://github.com/emilk/eframe_template");
                    ui.add(egui::github_link_file!(
                        "https://github.com/emilk/eframe_template/blob/master/",
                        "Source code."
                    ));
                    egui::warn_if_debug_build(ui);
                }
            }

            ui.add(Separator::default());
            ui.add_space(10.0);

            egui::ScrollArea::vertical()
                .always_show_scroll(false)
                .show(ui, |ui| {
                    let mut delet_id: Option<usize> = None;
                    for (id, entry) in entries.iter().enumerate() {
                        if entry.contains(search) {
                            TemplateApp::render_entry(entry, ui, title_color.color, name_color.color);
                            ui.horizontal(|ui| {
                                if ui.button("Pin").clicked() {
                                    *pined_entry = Some(id);
                                }
                                if ui.button("Delete").clicked() {
                                    delet_id = Some(id);
                                }
                            });
                            ui.add(Separator::default());
                        };
                    }
                    if let Some(id) = delet_id {
                        if *pined_entry == Some(id) {
                            *pined_entry = None;
                        }
                        drop(entries.remove(id));
                    };
                });
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Entry {
    name: String,
    owner: String,
    date: String,
    location: String,
    tags: Vec<Tag>,
    status: String,
}

impl Entry {
    fn new() -> Entry {
        Entry { name: String::new(), owner: String::new(), date: String::new(), location: String::new(), tags: vec![], status: String::new() }
    }

    fn from_str(name: &str, owner: &str, date: &str, location: &str, tags: &[Tag], status: &str) -> Entry {
        Entry { name: name.to_string(), owner: owner.to_string(), date: date.to_string(), location: location.to_string(), tags: tags.to_vec(), status: status.to_string() }
    }

    fn contains(&self, s: &str) -> bool {
        if self.name.to_lowercase().contains(&s.to_lowercase()) {
            return true;
        } else if self.date.to_lowercase().contains(&s.to_lowercase()) {
            return true;
        } else if self.location.to_lowercase().contains(&s.to_lowercase()) {
            return true;
        } else if self.owner.to_lowercase().contains(&s.to_lowercase()) {
            return true;
        } else if self.status.to_lowercase().contains(&s.to_lowercase()) {
            return true;
        } else if self.tags.contains(&Tag::from_str(s)) {
            return true;
        }
        false
    }

    fn clear(&mut self) {
        self.name = String::new();
        self.owner = String::new();
        self.date = String::new();
        self.location = String::new();
        self.tags = vec![];
        self.status = String::new();
    }
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
struct Color {
    color: Rgba,
    name: String,
}

impl Color {
    fn new() -> Color {
        Color { color: Rgba::BLACK, name: "Black".to_string() }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq)]
struct Tag {
    name: String,
}

impl Tag {
    fn from_str(s: &str) -> Tag {
        Tag { name: s.to_string() }
    }
}