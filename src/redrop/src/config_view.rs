use egui::Vec2;
use rfd::FileDialog;

use crate::ReDropApp;

// UI
impl ReDropApp {
    fn add_number_row<T: eframe::emath::Numeric>(
        &mut self,
        ui: &mut egui::Ui,
        name: &str,
        value: &mut T,
        min: f32,
        max: f32,
        step: f32,
    ) {
        ui.label(name);
        ui.add(
            egui::DragValue::new(value)
                .speed(step)
                .clamp_range(min..=max),
        );
        ui.end_row();
    }

    fn add_path_text_edit_row(&mut self, ui: &mut egui::Ui, name: &str, value: &mut String) {
        ui.label(name);
        ui.add(egui::TextEdit::singleline(value).min_size(Vec2::new(300., 0.))); // TODO: Maybe use desired_sise f32::INFINITY ?!
        if ui.button("Open").clicked() {
            // TODO: If path is in current directory, use relative path
            let path = std::path::Path::new(value);
            let mut absolute_path = path.to_path_buf();
            if !absolute_path.exists() {
                absolute_path = std::env::current_dir().unwrap();
            } else if path.is_relative() {
                absolute_path = std::env::current_dir().unwrap().join(path);
            }

            let directory = FileDialog::new()
                .set_directory(absolute_path)
                .set_title(format!("ReDrop - Select Folder for {}", name))
                .pick_folder();
            let selected = directory.unwrap(); // TODO: Handle error, or_default !?
            *value = selected.to_string_lossy().into_owned();
        }
        ui.end_row();
    }

    pub fn show_config_view(
        &mut self,
        // config_draft: &mut Config,
        ui: &mut egui::Ui,
        send_load_config_file: impl Fn(), // TODO: Self.send_load_config_file ?!
    ) {
        egui::Grid::new("config_grid")
            .num_columns(3)
            .spacing([16.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Window Size:");
                ui.end_row();
                let mut window_width = self.config_draft.window_width.clone();
                self.add_number_row(ui, "    Width:", &mut window_width, 100., 2000., 1.);
                let mut window_height = self.config_draft.window_height.clone();
                self.add_number_row(
                    ui,
                    "    Height:",
                    &mut window_height,
                    100.,
                    2000.,
                    1.,
                );

                ui.label("Mesh Size:");
                ui.end_row();
                self.add_number_row(
                    ui,
                    "    Width:",
                    &mut self.config_draft.mesh_width,
                    8.,
                    300.,
                    8.,
                ); // TODO : max, min, step ?!
                self.add_number_row(
                    ui,
                    "    Height:",
                    &mut self.config_draft.mesh_height,
                    8.,
                    300.,
                    2.,
                );

                self.add_number_row(
                    ui,
                    "Frame Rate:",
                    &mut self.config_draft.frame_rate,
                    1.,
                    144.,
                    1.,
                );

                self.add_path_text_edit_row(
                    ui,
                    "Presets Path:",
                    &mut self.config_draft.presets_path,
                );
                self.add_path_text_edit_row(
                    ui,
                    "Textures Path:",
                    &mut self.config_draft.textures_path,
                );

                self.add_number_row(
                    ui,
                    "Beat Sensitivity:",
                    &mut self.config_draft.beat_sensitivity,
                    0.1,
                    10.,
                    0.1,
                );

                self.add_number_row(
                    ui,
                    "Preset Duration:",
                    &mut self.config_draft.preset_duration,
                    1.,
                    60.,
                    1.,
                );
            });
        ui.add_space(8.0);
        ui.separator();
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            if ui.button("Save Config").clicked() {
                self.config.update_and_save(self.config_draft.to_owned());
                // self.update_and_save(config_draft.to_owned());
                send_load_config_file();
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close)
            }
            if ui.button("Reload Config").clicked() {
                // self.config.reload_config(&mut self.config_draft);
            }
            if ui.button("Restore Defaults").clicked() {
                self.config_draft.restore_defaults();
            }
        });
    }
}
