use crate::ReDropApp;
use common::ipc_message::Message;

impl ReDropApp {
    pub fn check_for_ipc_message(&mut self) {
        if let Some(ipc_from_child) = &mut self.ipc_from_child {
            if let Ok(message) = ipc_from_child.try_recv() {
                match message {
                    Message::RandomPresetRequest => {
                        self.send_random_preset_file();
                    }
                    Message::SwitchPresetRequest { smooth } => {
                        self.send_switch_preset_request(smooth);
                    }
                    Message::SetBeatSensitivity(sensitivity) => {
                        self.config.beat_sensitivity = sensitivity;
                    }
                    other_message => {
                        panic!("Unhandled message: {:?}", other_message);
                    }
                }
            }
        }
    }

    pub fn send_random_preset_file(&mut self) {
        let preset_id = rand::Rng::gen_range(&mut rand::thread_rng(), 0..self.presets.lists.len()); // TODO: Import rand::rng for simplifying the code
        self.send_load_preset_file(preset_id, self.smooth);
    }

    pub fn send_switch_preset_request(&self, smooth: bool) {
        let preset_id = rand::Rng::gen_range(&mut rand::thread_rng(), 0..self.presets.lists.len()); // TODO: Import rand::rng for simplifying the code
        self.send_load_preset_file(preset_id, smooth);
    }

    pub fn send_load_preset_file(&self, preset_id: usize, smooth: bool) {
        self.ipc_to_child
            .as_ref()
            .unwrap()
            .send(Message::LoadPresetFile {
                path: self.presets.lists[preset_id].path.clone(),
                smooth,
            })
            .unwrap();
    }

    pub fn send_load_config_file(&self) {
        self.ipc_to_child
            .as_ref()
            .unwrap()
            .send(Message::LoadConfigFile)
            .unwrap();
    }
}
