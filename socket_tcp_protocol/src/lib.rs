use std::fmt;
use serde::{Serialize, Deserialize};
// use serde_binary;

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    TurnOff,
    TurnOn,
    IsEnabled,
    GetCurrent,
    Unknown,
}


#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Ok,
    Enabled,
    Disabled,
    Current(f32),
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message{
    Command(Command),
    Response(Response)
}

pub fn serialize_message( msg: &Message) -> Vec<u8> {
    serde_json::to_vec(msg).unwrap()
}

pub fn deserialize_message(data:  &[u8]) -> Message {
    serde_json::from_slice(data).unwrap()
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}