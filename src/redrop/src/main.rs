use eframe::egui;

use std::path::Path;

use crate::ipc_message::{IpcExchange, Message};
use ipc_channel::ipc::IpcOneShotServer;

use common::config;
use common::ipc_message;
use common::preset;

mod config_view;
mod ipc;
mod main_ui;
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

    fn show_config_window(&mut self, ctx: &egui::Context) {
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

        self.show_main_ui(ctx);

        if self.show_config {
            self.show_config_window(ctx);
        }
    }
}
