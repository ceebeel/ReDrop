use eframe::egui;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

pub type FrameRate = u32;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "ReDrop",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(ReDropApp::new())
        }),
    )?;

    Ok(())
}

#[allow(dead_code)]
#[derive(Debug)]
struct Preset {
    // index: usize, //TODO: Remove iy unused
    name: String,
    path: PathBuf,
    img: Option<PathBuf>,
}

#[derive(Debug)]
enum Node {
    PresetId(usize),
    Directory(BTreeMap<String, Node>),
}

#[derive(Default)]
struct ReDropApp {
    presets: Vec<Preset>,
    presets_tree: BTreeMap<String, Node>,
}

impl ReDropApp {
    fn new() -> Self {
        let mut slf = Self::default();
        slf.update_presets_tree();
        slf
    }

    fn scan_presets(&mut self, path: &Path) -> BTreeMap<String, Node> {
        let mut dir = BTreeMap::new();
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                let mut sub_dir = BTreeMap::new();
                sub_dir.extend(self.scan_presets(&path));
                dir.insert(
                    path.file_name().unwrap().to_string_lossy().into_owned(),
                    Node::Directory(sub_dir),
                );
            } else if let Some(extension) = path.extension() {
                if extension == "milk" {
                    let name = path.file_stem().unwrap().to_string_lossy().into_owned();
                    let img = path.with_extension("jpg");
                    let preset_id = self.presets.len();
                    let preset = Preset {
                        // index: preset_id, //TODO: Remove iy unused
                        name: name.clone(),
                        path: path.clone(),
                        img: if img.exists() { Some(img) } else { None },
                    };
                    self.presets.push(preset);
                    dir.insert(name, Node::PresetId(preset_id));
                }
            }
        }
        dir
    }

    fn update_presets_tree(&mut self) {
        self.presets.clear();
        // TODO: Take presets path from config
        self.presets_tree = self.scan_presets(Path::new("Presets"));
    }

    fn render_presets_tree(&self, ui: &mut egui::Ui, dir: &BTreeMap<String, Node>) {
        for (name, node) in dir {
            match node {
                Node::PresetId(preset_id) => {
                    let preset = &self.presets[*preset_id];
                    if let Some(img_path) = &preset.img {
                        let file_path = "file://".to_owned() + img_path.to_str().unwrap();
                        // TODO: Add image button into a Grid (Responsive ?)
                        if ui
                            .add(egui::ImageButton::new(
                                egui::Image::new(file_path).max_size(egui::Vec2::new(64.0, 64.0)),
                            ))
                            .clicked()
                        {
                            println!("Preset:{:#?}", preset);
                        }
                    } else if ui.button(&preset.name).clicked() {
                        println!("Preset:{:#?}", preset);
                    }
                }
                Node::Directory(sub_dir) => {
                    egui::CollapsingHeader::new(name).show(ui, |ui| {
                        self.render_presets_tree(ui, sub_dir);
                    });
                }
            }
        }
    }
}

impl eframe::App for ReDropApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // TODO: Add Scroll
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_presets_tree(ui, &self.presets_tree);
        });
    }
}
