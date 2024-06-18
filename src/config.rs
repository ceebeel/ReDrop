use egui::Vec2;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub type FrameRate = u32;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub window_width: f32,     // Default: 800
    pub window_height: f32,    // Default: 600
    pub frame_rate: FrameRate, // Default: 60 fps
    pub presets_path: String,  // Default: ./presets
    pub textures_path: String, // Default: ./textures
    pub beat_sensitivity: f32, // Default: 1.0
    pub preset_duration: f64,  // Default: 60 (in seconds)
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_width: 800.,
            window_height: 600.,
            frame_rate: 60,
            presets_path: "Presets".to_string(),
            textures_path: "Textures".to_string(),
            beat_sensitivity: 1.,
            preset_duration: 5., // TODO: Restore to default (60)
        }
    }
}

impl Config {
    pub fn load_from_file_or_default(path: &PathBuf) -> Self {
        std::fs::read_to_string(path)
            .map(|data| toml::from_str(&data).unwrap_or_default())
            .unwrap_or_default()
    }

    #[allow(dead_code)]
    pub fn save_to_file(&self, path: &PathBuf) {
        let data = toml::to_string(&self).unwrap();
        std::fs::write(path, data).unwrap();
    }

    // pub fn save(&mut self, config: config) {

    // }

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
        let _ = ui.button("Open");
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
            let _ = ui.button("Save Config");
            let _ = ui.button("Reload Config");
            let _ = ui.button("Restore Defaults");
        });
    }
}
