/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    password_length: i32,
    password: String,
    
    #[serde(skip)] // This how you opt-out of serialization of a field
    allowed_chars: crate::AllowedSymbols,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            password_length: 10,
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
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        
        //use egui::FontFamily::Proportional;
        //use egui::FontId;
        //use egui::TextStyle::*;

        let mut style = (*cc.egui_ctx.style()).clone();
        /*
        style.text_styles = [
            (Heading, FontId::new(30.0, Proportional)),
            (Name("Heading2".into()), FontId::new(25.0, Proportional)),
            (Name("Context".into()), FontId::new(20.0, Proportional)),
            (Body, FontId::new(18.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (Button, FontId::new(14.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
            ].into();
        */
        style.spacing.slider_width = 200.0;
        cc.egui_ctx.set_style(style);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

fn selectable_text(ui: &mut egui::Ui, mut text: &str) {
    ui.add(egui::TextEdit::multiline(&mut text)
        .clip_text(false)
        .desired_width(f32::INFINITY)
    );
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        let mut window_width = 0.0;
        if let Some(rect) = ctx.input(|i| i.viewport().outer_rect) {
            window_width = rect.max.x - rect.min.x;
        }


        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().spacing.slider_width = window_width - 30.0;

            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Buttress - Password Generator");
            
            ui.add_space(20.0);
            ui.horizontal(|ui| {
                ui.label( "Password Length:");
                ui.label( self.password_length.to_string());
            });

            ui.add(egui::Slider::new(&mut self.password_length, 0..=100).show_value(false));
            
            ui.add_space(20.0);
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.allowed_chars.lower, "Lowercase");
                ui.add_space(20.0);
                ui.checkbox(&mut self.allowed_chars.upper, "Uppercase");
            });

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.allowed_chars.numbers, "Numbers");
                ui.add_space(29.0);
                ui.checkbox(&mut self.allowed_chars.special, "Special");
            });

            
            ui.add_space(20.0);
            ui.label( "Password");
            selectable_text(ui, &mut self.password);
            
            //ui.add_space(20.0);
            ui.horizontal(|ui| {
                if ui.button("GENERATE PASSWORD").clicked() {
                    self.password = crate::generate_password(self.password_length.try_into().unwrap(), &self.allowed_chars);
                };
        
                if ui.button("Copy to Clipboard").clicked() {
                    ui.output_mut(|o| o.copied_text = self.password.to_string());
                };
            });


            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/main/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
