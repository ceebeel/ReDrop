use ipc_channel::ipc::{IpcReceiver, IpcSender};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    LoadPresetFile { path: PathBuf, smooth: bool },
    RandomPresetRequest,                  // -> LoadPresetFile
    SwitchPresetRequest { smooth: bool }, // -> LoadPresetFile
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpcExchange {
    pub sender: IpcSender<Message>,
    pub receiver: IpcReceiver<Message>,
}
