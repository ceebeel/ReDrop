use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Preset {
    pub id: usize,
    pub name: String,
    pub path: PathBuf, //TODO: Change to Imutable Path ?!
    pub img: Option<PathBuf>,
}

pub enum Node {
    PresetId(usize),
    InnerNode(BTreeMap<String, Node>),
}

#[derive(Default)]
pub struct Presets {
    pub lists: Vec<Preset>,
    pub tree: BTreeMap<String, Node>,
}

impl Presets {
    pub fn update_presets_lists_and_tree(&mut self, path: &Path) {
        self.lists.clear();
        self.lists.clear();
        self.tree = self.scan_presets(path);
    }

    fn scan_presets(&mut self, path: &Path) -> BTreeMap<String, Node> {
        let mut node = BTreeMap::new();
        for entry in fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
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
                let preset_id = self.lists.len();
                let preset = Preset {
                    id: preset_id,
                    name: name.clone(),
                    path: path.clone(),
                    img: if img.exists() { Some(img) } else { None },
                };
                node.insert(name, Node::PresetId(preset_id));
                self.lists.push(preset);
            }
        }
        node
    }
}
