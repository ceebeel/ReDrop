use egui::Vec2;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub window_width: f32,     // Default: 800
    pub window_height: f32,    // Default: 600
    pub mesh_width: u32,       // Default: 32
    pub mesh_height: u32,      // Default: 24
    pub frame_rate: u32,       // Default: 60 fps
    pub presets_path: String,  // Default: ./presets
    pub textures_path: String, // Default: ./textures
    pub beat_sensitivity: f32, // Default: 1.0
    pub preset_duration: f64,  // Default: 60 (in seconds)
    pub shortcuts: Shortcuts,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_width: 800.,
            window_height: 600.,
            mesh_width: 32,
            mesh_height: 24,
            frame_rate: 60,
            presets_path: "Presets".to_string(),
            textures_path: "Textures".to_string(),
            beat_sensitivity: 1.,
            preset_duration: 5., // TODO: Restore to default (60)
            shortcuts: Shortcuts::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Shortcuts {
    pub toggle_fullscreen: egui::Key, // TODO: Double Click
    pub disable_fullscreen: egui::Key,
    pub next_preset: egui::Key,
    pub prev_preset: egui::Key,
    pub random_preset: egui::Key,
    pub beat_sensitivity_up: egui::Key,
    pub beat_sensitivity_down: egui::Key,
    // pub preset_speed_up: egui::Key,
    // pub preset_speed_down: egui::Key,
    pub rating_up: egui::Key,
    pub rating_down: egui::Key,
}

impl Default for Shortcuts {
    fn default() -> Self {
        Self {
            toggle_fullscreen: egui::Key::F,
            disable_fullscreen: egui::Key::Escape,
            next_preset: egui::Key::N,
            prev_preset: egui::Key::P,
            random_preset: egui::Key::R,
            beat_sensitivity_up: egui::Key::ArrowUp,
            beat_sensitivity_down: egui::Key::ArrowDown,
            // preset_speed_up: egui::Key::ArrowLeft,
            // preset_speed_down: egui::Key::ArrowRight,
            rating_up: egui::Key::Plus,
            rating_down: egui::Key::Minus,
        }
    }
}

impl Config {
    pub fn load_from_file_or_default() -> Self {
        std::fs::read_to_string("config.toml")
            .map(|data| toml::from_str(&data).unwrap_or_default())
            .unwrap_or_default()
    }

    pub fn save_to_file(&self) {
        let data = toml::to_string(&self).unwrap();
        std::fs::write("config.toml", data).unwrap();
    }

    fn update_and_save(&mut self, config_draft: Config) {
        *self = config_draft;
        self.save_to_file();
    }

    fn reload_config(&self, config_draft: &mut Config) {
        *config_draft = self.clone();
    }

    fn restore_defaults(&self, config_draft: &mut Config) {
        *config_draft = Config::default();
    }

    // UI
    fn add_number_row<T: eframe::emath::Numeric>(
        &mut self,
        ui: &mut egui::Ui,
        name: &str,
        value: &mut T,
        min: f32,
        max: f32,
        step: f32,
    ) {
        ui.label(name);
        ui.add(
            egui::DragValue::new(value)
                .speed(step)
                .clamp_range(min..=max),
        );
        ui.end_row();
    }

    fn add_path_text_edit_row(&mut self, ui: &mut egui::Ui, name: &str, value: &mut String) {
        ui.label(name);
        ui.add(egui::TextEdit::singleline(value).min_size(Vec2::new(300., 0.))); // TODO: Maybe use desired_sise f32::INFINITY ?!
        if ui.button("Open").clicked() {
            // TODO: If path is in current directory, use relative path
            let path = std::path::Path::new(value);
            let mut absolute_path = path.to_path_buf();
            if !absolute_path.exists() {
                absolute_path = std::env::current_dir().unwrap();
            } else if path.is_relative() {
                absolute_path = std::env::current_dir().unwrap().join(path);
            }

            let directory = FileDialog::new()
                .set_directory(absolute_path)
                .set_title(format!("ReDrop - Select Folder for {}", name))
                .pick_folder();
            let selected = directory.unwrap(); // TODO: Handle error, or_default !?
            *value = selected.to_string_lossy().into_owned();
        }
        ui.end_row();
    }

    #[allow(dead_code)]
    pub fn show(&mut self, config_draft: &mut Config, ui: &mut egui::Ui) {
        egui::Grid::new("config_grid")
            .num_columns(3)
            .spacing([16.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Window Size:");
                ui.end_row();
                self.add_number_row(
                    ui,
                    "    Width:",
                    &mut config_draft.window_width,
                    100.,
                    2000.,
                    1.,
                );
                self.add_number_row(
                    ui,
                    "    Height:",
                    &mut config_draft.window_height,
                    100.,
                    2000.,
                    1.,
                );

                ui.label("Mesh Size:");
                ui.end_row();
                self.add_number_row(ui, "    Width:", &mut config_draft.mesh_width, 8., 300., 8.); // TODO : max, min, step ?!
                self.add_number_row(
                    ui,
                    "    Height:",
                    &mut config_draft.mesh_height,
                    8.,
                    300.,
                    2.,
                );

                self.add_number_row(
                    ui,
                    "Frame Rate:",
                    &mut config_draft.frame_rate,
                    1.,
                    144.,
                    1.,
                );

                self.add_path_text_edit_row(ui, "Presets Path:", &mut config_draft.presets_path);
                self.add_path_text_edit_row(ui, "Textures Path:", &mut config_draft.textures_path);

                self.add_number_row(
                    ui,
                    "Beat Sensitivity:",
                    &mut config_draft.beat_sensitivity,
                    0.1,
                    10.,
                    0.1,
                );

                self.add_number_row(
                    ui,
                    "Preset Duration:",
                    &mut config_draft.preset_duration,
                    1.,
                    60.,
                    1.,
                );
            });
        ui.add_space(8.0);
        ui.separator();
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            if ui.button("Save Config").clicked() {
                self.update_and_save(config_draft.to_owned());
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close)
            }
            if ui.button("Reload Config").clicked() {
                self.reload_config(config_draft);
            }
            if ui.button("Restore Defaults").clicked() {
                self.restore_defaults(config_draft);
            }
        });
    }
}
