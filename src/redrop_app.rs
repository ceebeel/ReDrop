use eframe::egui;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
 
use ipc_channel::ipc::IpcOneShotServer;
use crate::ipc_message::{IpcExchange, Message};

pub type FrameRate = u32;

mod ipc_message;

mod config;
mod preset;

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
    player_app: Option<std::process::Child>,
}

impl ReDropApp {
    fn new() -> Self {
        let mut slf = Self::default();
        slf.config = config::Config::load_from_file_or_default(&PathBuf::from("./config.toml"));
        slf.config_draft = slf.config.clone();
        slf.presets
            .update_presets_lists_and_tree(Path::new(&slf.config.presets_path));

        slf.run_player_app();
        slf
    }

    fn run_player_app(&mut self, ) {
        self.player_app = Some(
            std::process::Command::new(
                std::env::current_exe()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join("redrop-player.exe"),
            )
            .spawn()
            // TODO: Add IPC Channel Name argument
            .unwrap(),
        );
    }

    fn send_load_preset_request(&self, preset_id: usize) {
        println!("Load preset: {:#?}", preset_id);
    }

    // UI
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
                self.send_load_preset_request(preset.id)
            }
        } else if ui.button(&preset.name).clicked() {
            self.send_load_preset_request(preset.id)
            // TODO: Button must be square
            // TODO: Idea: Create preview image on Right Click
        }
    }

    fn show_presets_tree(&self, ui: &mut egui::Ui, node: &BTreeMap<String, preset::Node>) {
        for (name, node) in node {
            match node {
                preset::Node::PresetId(preset_id) => {
                    self.show_preset(ui, preset_id);
                }
                preset::Node::InnerNode(inner_node) => {
                    egui::CollapsingHeader::new(name).show(ui, |ui| {
                        self.show_presets_tree(ui, inner_node);
                    });
                }
            }
        }
    }

    fn show_config(&mut self, ctx: &egui::Context) {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("config_immediate_viewport"),
            egui::ViewportBuilder::default()
                .with_title("ReDrop Config")
                .with_window_level(egui::WindowLevel::AlwaysOnTop)
                .with_resizable(false)
                .with_inner_size([480.0, 236.0]),
            |ctx, _class| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.config.show(&mut self.config_draft, ui);
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
        if self.show_config {
            self.show_config(ctx);
        }

        // TODO: Add Scroll
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
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
                    if let Some(player_app) = &mut self.player_app {
                        match player_app.try_wait() {
                            Ok(Some(_)) => self.run_player_app(),
                            Ok(None) => {
                                let _ = player_app.kill();
                                self.run_player_app();
                            }
                            Err(_) => todo!(),
                        }
                    }
                }
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_presets_tree(ui, &self.presets.tree); // TODO: Move presets_tree in the fn/
        });
    }
}
