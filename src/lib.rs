pub use bevy::prelude::*;
pub use bevy_renet::renet::*;
pub use bevy_renet::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    Ping,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    Pong,
}

pub const PROTOCOL_ID: u64 = 1000;
