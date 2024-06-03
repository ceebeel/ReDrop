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
        // Box::new(|_cc| Box::<ReDropApp>::default()),
        Box::new(|_cc| Box::new(ReDropApp::new(config))),
    )?;
    Ok(())
}

struct ReDropApp {
    project_m: ProjectMWrapped,
    config: config::Config,
    audio: audio::Audio,
    init: bool, // TODO : Fix: call function after Box::<ReDropApp>::default()
}

// impl Default for ReDropApp {
//     fn default() -> Self {
//         let project_m = Arc::new(ProjectM::create());
//         let config = config::Config::default();
//         Self { project_m, config, init: false }
//     }
// }

impl ReDropApp {
    fn new(config: config::Config) -> Self {
        let project_m = Arc::new(ProjectM::create());
        let audio = audio::Audio::new(Arc::clone(&project_m));
        // TODO: Option: Skip ProjetM default preset.
        // TODO: Remove Test preset 
        project_m.load_preset_file("./presets/! Test/reactive.milk", false);

        Self {
            project_m,
            config,
            audio,
            init: false,
        }
    }

    fn init(&mut self) {
        self.load_config(&self.config);
        let audio = self.audio.clone();
        std::thread::spawn(move || audio.capture_audio()); // TODO : arg: frame rate
        self.init = true;
    }
}

impl eframe::App for ReDropApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.init {
            self.init()
        }
        self.project_m.render_frame();
        ctx.request_repaint(); // TODO: Check if sync with frame rate
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.audio.is_capturing = false;
    }
    // TODO: FIX: Segmentation fault (after on_exit) -> Check with dock exemple code
}
