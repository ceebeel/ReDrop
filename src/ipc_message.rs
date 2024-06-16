use ipc_channel::ipc::{IpcReceiver, IpcSender};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    ClientToServer {
        message: String,
    },
    ServerToClient {
        message: String,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpcExchange {
    pub sender: IpcSender<Message>,
    pub receiver: IpcReceiver<Message>,
}