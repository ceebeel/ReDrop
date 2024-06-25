// use egui::Vec2;
// use rfd::FileDialog;
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
            preset_duration: 60., // TODO: Restore to default (60)
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

    fn save_to_file(&self) {
        let data = toml::to_string(&self).unwrap();
        std::fs::write("config.toml", data).unwrap();
    }

    pub fn update_and_save(&mut self, config_draft: Config) {
        *self = config_draft;
        self.save_to_file();
    }

    pub fn reload_config(&self, config_draft: &mut Config) {
        *config_draft = self.clone();
    }

    pub fn restore_defaults(&mut self) {
        *self = Config::default();
    }

}
