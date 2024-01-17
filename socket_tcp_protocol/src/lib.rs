use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    TurnOff,
    TurnOn,
    IsEnabled,
    GetPower,
    Unknown,
}


#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Ok,
    Enabled,
    Disabled,
    Power(f32),
    Unknown,
}


impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Response::Ok => write!(f, "Ok"),
            Response::Enabled => write!(f, "Enabled"),
            Response::Disabled => write!(f, "Disabled"),
            Response::Power(power) => write!(f, "Power: {}", power),
            Response::Unknown => write!(f, "Unknown"),
        }
    }
}