use std::path::PathBuf;

use crate::ReDropApp;
use serde::Deserialize;
use serde::Serialize;

pub type FrameRate = u32;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub window_width: f32,     // Default: 800
    pub window_height: f32,    // Default: 600
    pub frame_rate: FrameRate, // Default: 60 fps
    pub presets_path: String,  // Default: ./presets
    pub textures_path: String, // Default: ./textures
    pub beat_sensitivity: f32, // Default: 1.0
    pub preset_duration: f64,  // Default: ? (in seconds)
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_width: 800.,
            window_height: 600.,
            frame_rate: 60,
            presets_path: "./presets".to_string(),
            textures_path: "./textures".to_string(),
            beat_sensitivity: 1.,
            preset_duration: 10.,
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
