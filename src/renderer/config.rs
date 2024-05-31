use crate::ReDropApp;

pub type FrameRate = u32;

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

impl ReDropApp {
    pub fn load_config(&self, config: &Config) {
        let project_m = &self.project_m;

        if let Some(window_width) = config.window_width {
            if let Some(window_height) = config.window_height {
                project_m.set_window_size(window_width as usize, window_height as usize);
            }
        }

        if let Some(frame_rate) = config.frame_rate {
            project_m.set_fps(frame_rate);
        }

        if let Some(textures_path) = &config.textures_path {
            let paths = vec![textures_path.into()];
            project_m.set_texture_search_paths(&paths, 1)
        }

        // if let Some(presets_path) = &config.presets_path {
        // TODO: Use Playlist (Add index to projectm crate) or Custom Presets Manager
        // }

        if let Some(beat_sensitivity) = config.beat_sensitivity {
            project_m.set_beat_sensitivity(beat_sensitivity);
        }

        if let Some(preset_duration) = config.preset_duration {
            project_m.set_preset_duration(preset_duration);
        }
    }
}
