use socket_udp_protocol::*;
use std::io;
use std::io::{Read, Write};
use std::net::Stream;

fn main() {
    let mut stream = Stream::connect("127.0.0.1:8809").unwrap();

    loop {
        show_menu();
        let input = read_input();

        match input {
            Some(command) => {
                let payload = serialize_message(&command);
                let payload_size: u32 = payload.len() as u32;
                let size_slice: [u8; 4] = payload_size.to_be().to_be_bytes();
                stream.write_all(&size_slice).unwrap();
                stream.write_all(&payload).unwrap();

                let mut in_size_buffer = [0u8; 4];
                stream.read_exact(&mut in_size_buffer).unwrap();
                let size: usize = u32::from_be_bytes(in_size_buffer) as usize;
                let mut response_payload = vec![0; size];
                stream.read_exact(&mut response_payload[..]).unwrap();
                let response = deserialize_message(&response_payload[..]);

                println!("send: {command} -> {response}");
            }
            None => {
                println!("Bye...");
                break;
            }
        };
    }
}

fn show_menu() {
    println!();
    println!("------------------");
    println!("Select action:");
    println!("1) turn off");
    println!("2) turn on");
    println!("3) is enabled");
    println!("4) current");
    println!("_) exit");
}

fn read_input() -> Option<Message> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let cmd = match input.trim() {
        "1" => Message::Command(Command::TurnOff),
        "2" => Message::Command(Command::TurnOn),
        "3" => Message::Command(Command::IsEnabled),
        "4" => Message::Command(Command::GetCurrent),
        _ => return None,
    };
    Some(cmd)
}
