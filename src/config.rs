use std::path::PathBuf;

use crate::ReDropApp;
use serde::Deserialize;
use serde::Serialize;

pub type FrameRate = u32;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub window_width: Option<f32>,     // Default: 800
    pub window_height: Option<f32>,    // Default: 600
    pub frame_rate: Option<FrameRate>, // Default: 60 fps
    pub textures_path: Option<String>, // Default: ./textures
    pub presets_path: Option<String>,  // Default: ./presets
    pub beat_sensitivity: Option<f32>, // Default: 1.0
    pub preset_duration: Option<f64>,  // Default: ? (in seconds)
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_width: Some(800.),
            window_height: Some(600.),
            frame_rate: Some(60),
            textures_path: Some("./textures".to_string()),
            presets_path: Some("./presets".to_string()),
            beat_sensitivity: Some(1.),
            preset_duration: Some(10.),
        }
    }
}

impl Config {
    pub fn load_from_file_or_default(path: &PathBuf) -> Self {
        std::fs::read_to_string(path)
            .map(|data| toml::from_str(&data).unwrap_or_default())
            .unwrap_or_default()
    }

    pub fn save_to_file(&self, path: &PathBuf) {
        let data = toml::to_string(&self).unwrap();
        std::fs::write(path, data).unwrap();
    }
}

impl ReDropApp {

}
