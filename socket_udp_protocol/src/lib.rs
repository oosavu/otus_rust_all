use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub enum Message {
    Temperature(f32)
}

pub fn serialize_message(msg: &Message) -> Vec<u8> {
    serde_json::to_vec(msg).unwrap()
}

pub fn deserialize_message(data: &[u8]) -> Message {
    serde_json::from_slice(data).unwrap()
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
