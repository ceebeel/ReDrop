use crate::PlayerApp;
use common::config;
use common::ipc_message::IpcExchange;
use common::ipc_message::Message;
use ipc_channel::ipc::IpcSender;

pub fn ipc_connect() -> (ipc_channel::ipc::IpcReceiver<Message>, IpcSender<Message>) {
    let args: Vec<String> = std::env::args().collect();
    let sender = IpcSender::connect(args[1].clone()).unwrap();
    let (to_child, from_parent) = ipc_channel::ipc::channel().unwrap();
    let (to_parent, from_child) = ipc_channel::ipc::channel().unwrap();
    sender
        .send(IpcExchange {
            sender: to_child,
            receiver: from_child,
        })
        .unwrap();
    (from_parent, to_parent)
}

impl PlayerApp {
    pub fn check_for_ipc_message(&mut self) {
        if let Ok(message) = self.ipc_from_parent.try_recv() {
            match message {
                Message::LoadPresetFile { path, smooth } => self.load_preset_file(&path, smooth),
                Message::SetPresetDuration(duration) => {
                    println!("SetPresetDuration: {}", duration); // TODO: Remove this if fixed: too many request (Don't send request before release drag)
                    self.project_m.set_preset_duration(duration);
                }
                Message::LoadConfigFile => {
                    self.config = config::Config::load_from_file_or_default();
                    self.load_config(&self.config);
                }
                Message::SetBeatSensitivity(sensitivity) => {
                    println!("SetBeatSensitivity: {}", sensitivity); // TODO: Remove this if fixed: too many request (Don't send request before release drag)
                    self.project_m.set_beat_sensitivity(sensitivity);
                }
                other_message => {
                    panic!("Unhandled message: {:?}", other_message);
                }
            }
        }
    }

    pub fn send_random_preset_request(&self) {
        self.ipc_to_parent
            .send(Message::RandomPresetRequest)
            .unwrap();
    }
}
