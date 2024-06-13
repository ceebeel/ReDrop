use config::Config;
use eframe::egui;
use projectm::core::ProjectM;

use std::sync::Arc;
pub type ProjectMWrapped = Arc<ProjectM>;

pub mod audio;
pub mod config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    // TODO: Put config in user home directory
    let config_path = std::path::PathBuf::from("./config.toml"); // "config.toml";
    let config = config::Config::load_from_file_or_default(&config_path);
    config.save_to_file(&std::path::PathBuf::from("./config.toml"));
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("ReDrop")
            .with_inner_size([config.window_width.unwrap(), config.window_height.unwrap()])
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "ReDrop",
        options,
        Box::new(|_cc| Box::new(ReDropApp::new(config))),
    )?;
    Ok(())
}

struct ReDropApp {
    project_m: ProjectMWrapped,
    config: config::Config,
    audio: audio::Audio,
    fullscreen: bool,
}

impl ReDropApp {
    fn new(config: config::Config) -> Self {
        let project_m = Arc::new(ProjectM::create());
        let audio = audio::Audio::new(Arc::clone(&project_m));
        // TODO: Option: Skip ProjetM default preset.
        project_m.load_preset_file("./presets/! Test/reactive.milk", false);

        let mut redrop_app = ReDropApp {
            project_m,
            config,
            audio,
            fullscreen: false,
        };
        redrop_app.init();
        redrop_app
    }

    fn init(&mut self) {
        self.load_config(&self.config);
        let audio = self.audio.clone();
        std::thread::spawn(move || audio.capture_audio()); // TODO : arg: frame rate
    }

    // TODO: Zoom on viewport VS resize viewport (project_m)
    fn toggle_fullscreen(&mut self, ctx: &egui::Context) {
        if self.fullscreen {
            ctx.send_viewport_cmd(egui::viewport::ViewportCommand::Fullscreen(false));
            self.project_m.set_window_size(
                self.config.window_width.unwrap() as usize,
                self.config.window_height.unwrap() as usize,
            );
            self.fullscreen = false;
        } else {
            ctx.send_viewport_cmd(egui::viewport::ViewportCommand::Fullscreen(true));

            // Resize viewport
            let monitor_size = ctx.input(|i| i.viewport().monitor_size);
            let width = monitor_size.unwrap().x as usize;
            let height = monitor_size.unwrap().y as usize;
            self.project_m.set_window_size(width, height);

            self.fullscreen = true;
        }
    }
    
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

impl eframe::App for ReDropApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.project_m.render_frame();
        ctx.request_repaint(); // TODO: Check if sync with frame rate

        if ctx.input(|i| i.key_pressed(egui::Key::F)) {
            self.toggle_fullscreen(ctx);
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.audio.is_capturing = false;
    }
}
