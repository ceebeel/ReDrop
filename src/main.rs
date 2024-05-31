use eframe::egui;
use std::path::Path;
use std::path::PathBuf;
pub type FrameRate = u32;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        // renderer: eframe::Renderer::Glow,
        // viewport: egui::ViewportBuilder::default()
        // .with_inner_size([800.0, 600.0]),
        // .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "ReDrop",
        options,
        Box::new(|_cc| Box::<ReDropApp>::default()),
    )?;
    Ok(())
}
#[derive(Debug)]
struct Preset {
    name: String,
    path: PathBuf,
    img: String,
    // category: String,
    // subcategory: String,
}

struct ReDropApp {
    presets: Vec<Preset>,
}

impl Default for ReDropApp {
    fn default() -> Self {
        let presets = ReDropApp::list_presets(Path::new("./presets"));
        print!("Presets: {:#?}", presets);

        ReDropApp { presets }
    }
}

impl ReDropApp {
    fn list_presets(path: &Path) -> Vec<Preset> {
        let mut presets = Vec::new();
        let path = Path::new(path);
        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_file() && path.extension().unwrap() == "milk" {
                presets.push(Preset {
                    name: path.file_stem().unwrap().to_str().unwrap().to_string(),
                    // path: path.to_str().unwrap().to_string(),
                    path,
                    img: "".to_string(),
                });
            }
        }
        presets
    }
}

impl eframe::App for ReDropApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.collapsing("Category 1", |ui| {
                ui.collapsing("SubCategory 1", |ui| {
                    ui.label("Preset Name");
                    ui.label("Preset Name");
                    ui.label("Preset Name");
                    // ...
                });
                ui.collapsing("SubCategory 2", |ui| {
                    ui.label("Preset Name");
                    ui.label("Preset Name");
                    // ...
                });
                // ...
            });
            // ...
        });
    }
}
