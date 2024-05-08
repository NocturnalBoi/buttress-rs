/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    label: String,
    password_length: i32,
    password: String,
    
    #[serde(skip)] // This how you opt-out of serialization of a field
    allowed_chars: crate::AllowedSymbols,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            password_length: 50,
            password: String::new(),
            allowed_chars: crate::AllowedSymbols {
                lower: true,
                upper: true,
                numbers: true,
                special: true,
            }
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

fn selectable_text(ui: &mut egui::Ui, mut text: &str, text_color: egui::Color32) {
    ui.add(egui::TextEdit::multiline(&mut text)
        .clip_text(false)
        .desired_width(f32::INFINITY)
        .desired_rows(5)
        .text_color(text_color)
    );
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut window_width = 0.0;
        if let Some(rect) = ctx.input(|i| i.viewport().outer_rect) {
            window_width = rect.max.x - rect.min.x;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().spacing.slider_width = window_width - 80.0;

            ui.horizontal(|ui| {
                ui.label( "Password");
                ui.add_space(window_width - 150.0);
                if ui.button("🔄")
                    .on_hover_text("Generate new password")
                    .clicked() {
                        self.password = crate::generate_password(self.password_length.try_into().unwrap(), &self.allowed_chars);
                };
        
                if ui.button("💾")
                    .on_hover_text("Copy to clipboard")
                    .clicked() {
                        ui.output_mut(|o| o.copied_text = self.password.to_string());
                };
            });

            ui.add_space(10.0);

            let mut password_color: egui::Color32 = egui::Color32::DARK_RED;
            if self.password_length > 8 {
                password_color = egui::Color32::from_rgb(144, 144, 0);
            }

            if self.password_length > 16 {
                password_color = egui::Color32::DARK_GREEN;
            } 

            selectable_text(ui, &mut self.password, password_color);
            ui.separator();
            
            ui.add_space(20.0);
            ui.horizontal(|ui| {
                ui.label( "Password Length:");
            });
            
            let slider_response = ui.add(egui::Slider::new(&mut self.password_length, 0..=100)
                .show_value(true)
            );
            
            if slider_response.changed() == true {
                self.password = crate::generate_password(self.password_length.try_into().unwrap(), &self.allowed_chars);
            };
            
            ui.add_space(20.0);
            ui.horizontal(|ui| {
                if ui.checkbox(&mut self.allowed_chars.lower, "Lowercase")
                    .changed() {
                        self.password = crate::generate_password(self.password_length.try_into().unwrap(), &self.allowed_chars);
                };

                ui.add_space(50.0);

                if ui.checkbox(&mut self.allowed_chars.upper, "Uppercase")
                    .changed() {
                        self.password = crate::generate_password(self.password_length.try_into().unwrap(), &self.allowed_chars);
                };
            });

            ui.horizontal(|ui| {
                if ui.checkbox(&mut self.allowed_chars.numbers, "Numbers")
                    .changed() {
                        self.password = crate::generate_password(self.password_length.try_into().unwrap(), &self.allowed_chars);
                };

                ui.add_space(59.0);

                if ui.checkbox(&mut self.allowed_chars.special, "Special Symbols")
                    .changed() {
                        self.password = crate::generate_password(self.password_length.try_into().unwrap(), &self.allowed_chars);
                };
            });
        });

        egui::TopBottomPanel::bottom("bot_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_buttons(ui);
                egui::warn_if_debug_build(ui);
                ui.add_space(window_width - 210.0);
                ui.add(egui::github_link_file!(
                    "https://github.com/NocturnalBoi/buttress-rs.git",
                    "Source code"
                ));
            });
        });
    }
}
