use egui::{Separator, Rgba};

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

    #[serde(skip)]
    setting_preferences: bool,
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

    fn render_entry(entry: &Entry, ui: &mut eframe::egui::Ui, color: Rgba) {
        ui.heading(&entry.title);
        ui.colored_label(color, &entry.desc);
        ui.label(&entry.content);
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

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
                ui.menu_button("View", |ui| {
                    if ui.button("Preference").clicked() {
                        *setting_preferences = true;
                    }
                });
            });
        });
        if *generating_entry {
            egui::Window::new("Create a new entry")
                .collapsible(false)
                .resizable(true)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Title: ");
                        ui.text_edit_singleline(&mut temp_entry.title);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Description: ");
                        ui.text_edit_singleline(&mut temp_entry.desc);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Content: ");
                        ui.text_edit_singleline(&mut temp_entry.content);
                    });
                    
                    ui.horizontal(|ui| {
                        if ui.button("Done").clicked() {
                            entries.push(Entry::from_str(
                                &temp_entry.title,
                                &temp_entry.desc,
                                &temp_entry.content,
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
                        egui::ComboBox::from_label("Pick one!")
                            .selected_text(&title_color.name)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(title_color, Color{ color: Rgba::BLACK,
                                    name: "Black".to_string()}, "Black");
                                ui.selectable_value(title_color, Color{ color: Rgba::BLUE,
                                    name: "Blue".to_string()}, "Blue");
                                ui.selectable_value(title_color, Color{ color: Rgba::RED,
                                    name: "Red".to_string()}, "Red");
                            })
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
                    TemplateApp::render_entry(entries.get(*id).unwrap(), ui, title_color.color);
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
            ui.add_space(5.0);

            egui::ScrollArea::vertical()
                .always_show_scroll(false)
                .show(ui, |ui| {
                    let mut delet_id: Option<usize> = None;
                    for (id, _) in entries.iter().enumerate() {
                        TemplateApp::render_entry(entries.get(id).unwrap(), ui, title_color.color);
                        ui.horizontal(|ui|{
                            if ui.button("Pin").clicked() {
                                *pined_entry = Some(id);
                            }
                            if ui.button("Delete").clicked() {
                                delet_id = Some(id);
                            }
                        });
                        
                        ui.add(Separator::default());
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
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Entry {
    title: String,
    desc: String,
    content: String,
}

impl Entry {
    fn new() -> Entry {
        Entry {
            title: "".to_string(),
            desc: "".to_string(),
            content: "".to_string(),
        }
    }

    fn from_str(title: &str, desc: &str, content: &str) -> Entry {
        Entry {
            title: title.to_string(),
            desc: desc.to_string(),
            content: content.to_string(),
        }
    }

    fn clear(&mut self) {
        self.title = "".to_string();
        self.desc = "".to_string();
        self.content = "".to_string();
    }
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
struct Color {
    color: Rgba,
    name: String,
}

impl Color {
    fn new() -> Color {
        Color { color: Rgba::BLACK, name: "Black".to_string(), }
    }
}