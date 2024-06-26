use crate::ReDropApp;
use common::ipc_message::Message;

impl ReDropApp {
    pub fn show_main_ui(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.preset_search_query);
                if ui.button("Config").clicked() {
                    self.show_config = true;
                }
                if ui.button("Kill").clicked() {
                    if let Some(player_app) = &mut self.player_app {
                        let _ = player_app.kill();
                        // TODO: Don't kill it! Send a request to the player app to exit.
                    }
                }
                if ui.button("Run").clicked() {
                    if let Some(_player_app) = &mut self.player_app {
                        // match player_app.try_wait() {
                        //     Ok(Some(_)) => self.run_player_app(),
                        //     Ok(None) => {
                        //         let _ = player_app.kill();
                        //         self.run_player_app(); // TODO: Fix: server_name in ReDropApp or PlayerApp (new struct) !?
                        //     }
                        //     Err(_) => todo!(),
                        // }
                    }
                }
                ui.checkbox(&mut self.smooth, "Smooth");

                ui.label("Preset Duration:");
                let last_preset_duration = self.config.preset_duration;
                ui.add(
                    egui::DragValue::new(&mut self.config.preset_duration)
                        .clamp_range(0..=1000)
                        .update_while_editing(false), // TODO: update_while_editing don't work on drag value (mouse move), maybe check clicked/released !?
                );
                if last_preset_duration != self.config.preset_duration {
                    self.ipc_to_child
                        .as_ref()
                        .unwrap()
                        .send(Message::SetPresetDuration(self.config.preset_duration))
                        .unwrap();
                }

                ui.label("Beat Sensitivity:");
                let last_beat_sensitivity = self.config.beat_sensitivity;
                ui.add(
                    egui::DragValue::new(&mut self.config.beat_sensitivity)
                        .clamp_range(0.1..=10.)
                        .update_while_editing(false), // TODO: update_while_editing don't work on drag value (mouse move), maybe check clicked/released !?
                );
                if last_beat_sensitivity != self.config.beat_sensitivity {
                    self.ipc_to_child
                        .as_ref()
                        .unwrap()
                        .send(Message::SetBeatSensitivity(self.config.beat_sensitivity))
                        .unwrap();
                }
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| {
                    // self.show_presets_into_flat_tree(ui, &self.presets.tree);
                    let option_grid = true; // TODO: Option in config [ReDrop]
                    if option_grid {
                        self.show_presets_into_tree_grid(ui, &self.presets.tree);
                    // TODO: Move presets_tree in the fn
                    } else {
                        self.show_presets_into_flat_tree(ui, &self.presets.tree);
                        // TODO: Move presets_tree in the fn
                    }
                });
        });
    }
}
