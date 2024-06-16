use std::path::{Path, PathBuf};

use ipc_channel::ipc::{IpcReceiver, IpcSender};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    LoadPresetFile {
        path: PathBuf,
        smooth: bool,
    },
    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpcExchange {
    pub sender: IpcSender<Message>,
    pub receiver: IpcReceiver<Message>,
}