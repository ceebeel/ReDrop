use eframe::egui;

use std::path::Path;

use crate::ipc_message::{IpcExchange, Message};
use ipc_channel::ipc::IpcOneShotServer;

use common::config;
use common::ipc_message;
use common::preset;

mod config_view;
mod ipc;
mod presets_ui;

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

#[derive(Default)]
struct ReDropApp {
    config: config::Config,
    config_draft: config::Config,
    show_config: bool,
    presets: preset::Presets,
    smooth: bool,
    preset_search_query: String,
    player_app: Option<std::process::Child>,
    ipc_to_child: Option<ipc_channel::ipc::IpcSender<Message>>,
    ipc_from_child: Option<ipc_channel::ipc::IpcReceiver<Message>>,
}

impl ReDropApp {
    fn new() -> Self {
        let mut slf = Self::default();
        slf.config = config::Config::load_from_file_or_default();
        slf.config_draft = slf.config.clone();
        slf.presets
            .update_presets_lists_and_tree(Path::new(&slf.config.presets_path));

        let (ipc_server, server_name) = IpcOneShotServer::<IpcExchange>::new().unwrap();

        slf.run_player_app(server_name);

        let (_, IpcExchange { sender, receiver }) = ipc_server.accept().unwrap();
        slf.ipc_to_child = Some(sender);
        slf.ipc_from_child = Some(receiver);

        // slf.timer = Some(std::time::Instant::now());

        slf
    }

    fn run_player_app(&mut self, server_name: String) {
        self.player_app = Some(
            std::process::Command::new(
                std::env::current_exe()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join("redrop_player.exe"),
            )
            .arg(server_name)
            .spawn()
            .unwrap(),
        );
    }

    fn show_config(&mut self, ctx: &egui::Context) {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("config_immediate_viewport"),
            egui::ViewportBuilder::default()
                .with_title("ReDrop - Config")
                // .with_window_level(egui::WindowLevel::AlwaysOnTop)
                .with_resizable(false)
                .with_inner_size([480.0, 300.0]), // TODO: Auto size to fit content
            |ctx, _class| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.show_config_view(ui);
                });

                if ctx.input(|i| i.viewport().close_requested()) {
                    self.show_config = false;
                }
            },
        );
    }
}

impl eframe::App for ReDropApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.check_for_ipc_message();
        ctx.request_repaint();
        std::thread::sleep(std::time::Duration::from_millis(10)); // TODO: Spawn check_for_ipc_message in a thread (for not continuously updating the UI)

        if self.show_config {
            self.show_config(ctx);
        }

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
                        self.show_presets_into_tree_grid(ui, &self.presets.tree); // TODO: Move presets_tree in the fn
                    } else {
                        self.show_presets_into_flat_tree(ui, &self.presets.tree); // TODO: Move presets_tree in the fn
                    }
                });
        });
    }
}
