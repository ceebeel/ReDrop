use eframe::egui;

use std::path::Path;
use std::sync::Arc;

use projectm::core::ProjectM;
pub type ProjectMWrapped = Arc<ProjectM>;

use crate::ipc_message::{IpcExchange, Message};
use ipc_channel::ipc;
use ipc_channel::ipc::IpcSender;

use config::Config;

mod audio;
mod config;
mod frame_history;
mod ipc_message;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    // ipc-channel
    let args: Vec<String> = std::env::args().collect();
    let sender = IpcSender::connect(args[1].clone()).unwrap();
    let (to_child, from_parent) = ipc::channel().unwrap();
    let (to_parent, from_child) = ipc::channel().unwrap();
    sender
        .send(IpcExchange {
            sender: to_child,
            receiver: from_child,
        })
        .unwrap();

    let config = config::Config::load_from_file_or_default();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("ReDrop Player")
            .with_inner_size([config.window_width, config.window_height])
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "ReDrop",
        options,
        Box::new(|_cc| Box::new(PlayerApp::new(config, from_parent, to_parent))),
    )?;
    Ok(())
}

struct PlayerApp {
    project_m: ProjectMWrapped,
    config: config::Config,
    audio: audio::Audio,
    fullscreen: bool,
    ipc_from_parent: ipc_channel::ipc::IpcReceiver<Message>,
    ipc_to_parent: ipc_channel::ipc::IpcSender<Message>,
    current_preset_name: String,
    frame_history: frame_history::FrameHistory,
}

impl PlayerApp {
    fn new(
        config: config::Config,
        ipc_from_parent: ipc_channel::ipc::IpcReceiver<Message>,
        ipc_to_parent: ipc_channel::ipc::IpcSender<Message>,
    ) -> Self {
        let project_m = Arc::new(ProjectM::create());
        let audio = audio::Audio::new(Arc::clone(&project_m));
        // TODO: Option: Skip ProjectM default preset (load preset here before playing).

        let mut player_app = PlayerApp {
            project_m,
            config,
            audio,
            fullscreen: false,
            ipc_from_parent,
            ipc_to_parent,
            current_preset_name: String::default(),
            frame_history: frame_history::FrameHistory::default(),
        };
        player_app.init();
        player_app
    }

    fn init(&mut self) {
        self.load_config(&self.config);
        self.set_preset_swithch_request_callback();
        let audio = self.audio.clone();
        std::thread::spawn(move || audio.capture_audio()); // TODO : arg: frame rate
    }

    // ProjecM Callbacks
    fn set_preset_swithch_request_callback(&mut self) {
        let ipc_to_parent = self.ipc_to_parent.clone();
        self.project_m
            .set_preset_switch_requested_event_callback(move |cut| {
                ipc_to_parent
                    .send(Message::SwitchPresetRequest { smooth: cut })
                    .unwrap();
            });
    }

    // TODO: Zoom on viewport VS resize viewport (project_m) (maybe ctx.zoom_factor ?!)
    fn toggle_fullscreen(&mut self, ctx: &egui::Context) {
        if self.fullscreen {
            ctx.send_viewport_cmd(egui::viewport::ViewportCommand::Fullscreen(false));
            ctx.send_viewport_cmd(egui::viewport::ViewportCommand::CursorVisible(true));
            self.project_m.set_window_size(
                self.config.window_width as usize,
                self.config.window_height as usize,
            );
            self.fullscreen = false;
        } else {
            ctx.send_viewport_cmd(egui::viewport::ViewportCommand::Fullscreen(true));
            ctx.send_viewport_cmd(egui::viewport::ViewportCommand::CursorVisible(false));

            // Resize viewport
            let monitor_size = ctx.input(|i| i.viewport().monitor_size);
            let width = monitor_size.unwrap().x as usize;
            let height = monitor_size.unwrap().y as usize;
            self.project_m.set_window_size(width, height);

            self.fullscreen = true;
        }
    }

    pub fn load_config(&self, config: &Config) {
        let project_m = &self.project_m;

        project_m.set_window_size(config.window_width as usize, config.window_height as usize);
        project_m.set_mesh_size(config.mesh_width as usize, config.mesh_height as usize);
        project_m.set_fps(config.frame_rate);
        let paths = vec![config.textures_path.clone()];
        project_m.set_texture_search_paths(&paths, 1);
        project_m.set_beat_sensitivity(config.beat_sensitivity);
        project_m.set_preset_duration(config.preset_duration);
    }

    fn load_preset_file(&mut self, path: &Path, smooth: bool) {
        // project_m.load_preset_file does not work fine with special characters like spaces...
        self.current_preset_name = path.file_stem().unwrap().to_string_lossy().into_owned(); // TODO: .into_owned() !?
        let data = std::fs::read_to_string(path).unwrap();
        self.project_m.load_preset_data(&data, smooth);
    }

    // ipc-channel
    fn check_for_ipc_message(&mut self) {
        if let Ok(message) = self.ipc_from_parent.try_recv() {
            match message {
                Message::LoadPresetFile { path, smooth } => self.load_preset_file(&path, smooth),
                Message::SetPresetDuration(duration) => {
                    println!("SetPresetDuration: {}", duration); // TODO: Remove this if fixed: too many request (Don't send request before release drag)
                    self.project_m.set_preset_duration(duration);
                }
                other_message => {
                    panic!("Unhandled message: {:?}", other_message);
                }
            }
        }
    }

    fn send_random_preset_request(&self) {
        self.ipc_to_parent
            .send(Message::RandomPresetRequest)
            .unwrap();
    }
}

impl eframe::App for PlayerApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // let now = Instant::now(); // Fix sync
        self.frame_history
            .on_new_frame(ctx.input(|i| i.time), frame.info().cpu_usage);
        self.project_m.set_fps(self.frame_history.fps() as u32);

        if !self.fullscreen {
            let window_title = format!(
                "ReDrop - {:.0}fps - {:.2}ms - {}",
                self.frame_history.fps(),
                1e3 * self.frame_history.mean_frame_time(), // CPU Usage in ms / frame
                self.current_preset_name
            );
            ctx.send_viewport_cmd(egui::ViewportCommand::Title(window_title));
            // TODO: Set window title when preset is loaded (on fullscreen - no fps or cpu usage)
        }

        self.check_for_ipc_message();

        self.project_m.render_frame();
        ctx.request_repaint();

        if ctx.input(|i| {
            i.key_pressed(self.config.shortcuts.toggle_fullscreen) || i.pointer.any_click()
        }) {
            // TODO: Fix: any_click() to double click
            self.toggle_fullscreen(ctx);
        }

        if ctx.input(|i| i.key_pressed(self.config.shortcuts.disable_fullscreen)) && self.fullscreen
        {
            self.toggle_fullscreen(ctx);
        }

        if ctx.input(|i| i.key_pressed(self.config.shortcuts.random_preset)) {
            self.send_random_preset_request();
        }

        // self.frame_history.limit_fps(now); // TODO : Fix sync
    }
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.audio.is_capturing = false;
    }
}
