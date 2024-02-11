use rand::Rng;
use socket_tcp_protocol::*;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8809").await.unwrap();
    for _i in 0..10{
        let command = random_command();
        let payload = serialize_message(&command);
        let payload_size: u32 = payload.len() as u32;
        let size_slice: [u8; 4] = payload_size.to_be().to_be_bytes();
        stream.write_all(&size_slice).await.unwrap();
        stream.write_all(&payload).await.unwrap();

        let mut in_size_buffer = [0u8; 4];
        stream.read_exact(&mut in_size_buffer).await.unwrap();
        let size: usize = u32::from_be_bytes(in_size_buffer) as usize;
        let mut response_payload = vec![0; size];
        stream.read_exact(&mut response_payload[..]).await.unwrap();
        let response = deserialize_message(&response_payload[..]);

        println!("send: {command} -> {response}");
        
    }
}


fn random_command() -> Message {
    //TODO в расте нельзя вот просто взять и число преобразовать в enum.
    //есть решения которые это автоматизируют, но не в рамках домашнего задания
    let cmd_num: i32 = rand::thread_rng().gen_range(0..4);
    let cmd = match cmd_num {
        0 => Message::Command(Command::TurnOff),
        1 => Message::Command(Command::TurnOn),
        2 => Message::Command(Command::IsEnabled),
        3 => Message::Command(Command::GetCurrent),
        _ => return Message::Command(Command::TurnOff),
    };
    cmd
}
