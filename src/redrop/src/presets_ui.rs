use crate::ReDropApp;
use common::preset;
use std::collections::BTreeMap;

impl ReDropApp {
    pub fn show_presets_into_tree_grid(
        &self,
        ui: &mut egui::Ui,
        node: &BTreeMap<String, preset::Node>,
    ) {
        egui::Grid::new("preset_grid")
            .num_columns(1)
            .spacing([4., 4.])
            .show(ui, |ui| {
                let mut preset_count = 0;
                let max_preset_per_row = (ui.available_width() / 68.) as usize; // Preset width = 64. + 4. (spacing)
                for (name, node) in node {
                    match node {
                        preset::Node::PresetId(preset_id) => {
                            if self.presets.lists[*preset_id]
                                .name
                                .contains(&self.preset_search_query)
                            {
                                self.show_preset(ui, preset_id);
                                preset_count += 1;
                                if preset_count >= max_preset_per_row {
                                    ui.end_row();
                                    preset_count = 0;
                                }
                            }
                        }
                        preset::Node::InnerNode(inner_node) => {
                            egui::CollapsingHeader::new(name).show(ui, |ui| {
                                self.show_presets_into_tree_grid(ui, inner_node);
                            });
                            ui.end_row();
                        }
                    }
                }
            });
    }

    fn show_preset(&self, ui: &mut egui::Ui, preset_id: &usize) {
        // TODO: Add image button into a Grid (Responsive ?)
        let preset = &self.presets.lists[*preset_id];
        if let Some(img_path) = &preset.img {
            let file_path = "file://".to_owned() + img_path.to_str().unwrap();
            let image = egui::Image::new(&file_path).fit_to_exact_size(egui::Vec2::new(64., 64.));
            let image_hovered =
                egui::Image::new(&file_path).fit_to_exact_size(egui::Vec2::new(96., 96.));
            let image_button = egui::ImageButton::new(image).frame(false);
            let response = ui.add(image_button);

            if response.hovered() {
                let pos = response.rect.center() - egui::Vec2::new(48., 48.); // image_hovered size / 2
                let _area_response = egui::Area::new("hovered_image".into())
                    .fixed_pos(pos)
                    .order(egui::Order::Tooltip)
                    .show(ui.ctx(), |ui| {
                        ui.add(image_hovered);
                    });
            }
            if response.clicked() {
                self.send_load_preset_file(preset.id, self.smooth)
            }
        } else {
            // TODO: Idea: Create preview image (screenshot) on Right Click // Or all (scan) in Config View
            if ui
                .add_sized([64., 64.], egui::Button::new(&preset.name).wrap(true)) // TODO Fix: Button size "overflow" if name is too long / This can be a problem with grid..
                .clicked()
            {
                self.send_load_preset_file(preset.id, self.smooth)
            }
        }
    }

    pub fn show_presets_into_flat_tree(
        &self,
        ui: &mut egui::Ui,
        node: &BTreeMap<String, preset::Node>,
    ) {
        for (name, node) in node {
            match node {
                preset::Node::PresetId(preset_id) => {
                    self.show_preset_flat(ui, preset_id);
                }
                preset::Node::InnerNode(inner_node) => {
                    egui::CollapsingHeader::new(name).show(ui, |ui| {
                        self.show_presets_into_flat_tree(ui, inner_node);
                    });
                }
            }
        }
    }

    fn show_preset_flat(&self, ui: &mut egui::Ui, preset_id: &usize) {
        let preset = &self.presets.lists[*preset_id];
        if ui
            .add(egui::Button::new(&preset.name).wrap(true).frame(false))
            .clicked()
        {
            self.send_load_preset_file(preset.id, self.smooth)
        }
    }
}
