use eframe::egui;
use projectm::core::ProjectM;

use std::sync::Arc;
pub type ProjectMWrapped = Arc<ProjectM>;

pub mod audio;
pub mod config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let config = config::Config::default();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("ReDrop")
            .with_inner_size([config.window_width.unwrap(), config.window_height.unwrap()]),
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
        };
        redrop_app.init();
        redrop_app
    }

    fn init(&mut self) {
        self.load_config(&self.config);
        let audio = self.audio.clone();
        std::thread::spawn(move || audio.capture_audio()); // TODO : arg: frame rate
    }
}

impl eframe::App for ReDropApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.project_m.render_frame();
        ctx.request_repaint(); // TODO: Check if sync with frame rate
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.audio.is_capturing = false;
    }
}
