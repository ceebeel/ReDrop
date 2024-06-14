use std::{collections::BTreeMap, path::{Path, PathBuf}};

struct Preset {
    id: usize,
    name: String,
    path: PathBuf, // Mybe don't save it here // Save in server with (id, path)
    img: Option<PathBuf>,
}

enum Node {
    PresetId(usize),
    InnerNode(BTreeMap<String, Node>),
}

struct Presets {
    lists: Vec<Preset>,
    tree: BTreeMap<String, Node>,
}

impl Presets {
    fn new() -> Self {
        Self {
            lists: vec![],
            tree: BTreeMap::new(),
        }
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
}
