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

#[derive(Debug)]
struct Preset {
    id: usize,
    name: String,
    path: PathBuf,
    img: Option<PathBuf>,
}

#[derive(Debug)]
enum Node {
    PresetId(usize),
    InnerNode(BTreeMap<String, Node>),
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
        let mut node = BTreeMap::new();
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                let mut inner_node = BTreeMap::new();
                inner_node.extend(self.scan_presets(&path));
                node.insert(
                    path.file_name().unwrap().to_string_lossy().into_owned(),
                    Node::InnerNode(inner_node),
                );
            } else if path.extension().unwrap() == "milk" {
                let name = path.file_stem().unwrap().to_string_lossy().into_owned();
                let img = path.with_extension("jpg");
                let preset_id = self.presets.len();
                let preset = Preset {
                    id: preset_id,
                    name: name.clone(),
                    path: path.clone(),
                    img: if img.exists() { Some(img) } else { None },
                };
                node.insert(name, Node::PresetId(preset_id));
                self.presets.push(preset);
            }
        }
        node
    }

    fn update_presets_tree(&mut self) {
        self.presets.clear();
        self.presets_tree.clear();
        // TODO: Take presets path from config
        self.presets_tree = self.scan_presets(Path::new("Presets"));
    }

    fn send_load_preset_request(&self, preset_id: usize) {
        println!("Load preset: {:#?}", preset_id);
    }

    // UI
    fn show_preset(&self, ui: &mut egui::Ui, preset_id: &usize) {
        let preset = &self.presets[*preset_id];
        if let Some(img_path) = &preset.img {
            let file_path = "file://".to_owned() + img_path.to_str().unwrap();
            // TODO: Add image button into a Grid (Responsive ?)
            if ui
                .add(egui::ImageButton::new(
                    egui::Image::new(file_path).fit_to_exact_size(egui::Vec2::new(64.0, 64.0)),
                ))
                .clicked()
            {
                self.send_load_preset_request(preset.id)
            }
        } else if ui.button(&preset.name).clicked() {
            self.send_load_preset_request(preset.id)
        }
    }

    fn show_presets_tree(&self, ui: &mut egui::Ui, node: &BTreeMap<String, Node>) {
        for (name, node) in node {
            match node {
                Node::PresetId(preset_id) => {
                    self.show_preset(ui, preset_id);
                }
                Node::InnerNode(inner_node) => {
                    egui::CollapsingHeader::new(name).show(ui, |ui| {
                        self.show_presets_tree(ui, inner_node);
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
            self.show_presets_tree(ui, &self.presets_tree);
        });
    }
}
