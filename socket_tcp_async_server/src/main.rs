use std::sync::Arc;
use socket_tcp_protocol::*;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::Mutex,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8809").await.expect("can't bind tcp listener");

    let smart_socket: Arc<Mutex<SmartSocket>> = Arc::new(Mutex::new(SmartSocket::default()));

    loop {
        let (mut stream, peer) = listener.accept().await.unwrap();
        println!("Peer '{peer}' connected");
        let socket_clone = smart_socket.clone();

        tokio::spawn(async move {
            loop {
                let mut in_size_buffer = [0u8; 4];
                stream.read_exact(&mut in_size_buffer).await.unwrap();
                let size = u32::from_ne_bytes(in_size_buffer) as usize;
                let mut payload = vec![0; size];
                stream.read_exact(&mut payload[..]).await.unwrap();
                let msg = deserialize_message(payload.as_slice());

                let response = socket_clone.lock().await.process_message(&msg);
                println!("process messgae: {msg} -> {response}");
                let response_buf = serialize_message(&response);
                let response_size: u32 = response_buf.len() as u32;
                let size_slice: [u8; 4] = response_size.to_be().to_ne_bytes();
                if stream.write_all(&size_slice).await.is_err() {
                    break;
                };
                if stream.write_all(&response_buf).await.is_err() {
                    break;
                };
            }
        });
    }
}

#[derive(Default)]
struct SmartSocket {
    enabled: bool,
}

impl SmartSocket {
    fn process_message(&mut self, msg: &Message) -> Message {
        match msg {
            Message::Command(cmd) => match cmd {
                Command::TurnOn => {
                    self.enabled = true;
                    Message::Response(Response::Ok)
                }
                Command::TurnOff => {
                    self.enabled = false;
                    Message::Response(Response::Ok)
                }
                Command::IsEnabled => {
                    if self.enabled {
                        Message::Response(Response::Enabled)
                    } else {
                        Message::Response(Response::Disabled)
                    }
                }
                Command::GetCurrent => {
                    if self.enabled {
                        Message::Response(Response::Current(220.0))
                    } else {
                        Message::Response(Response::Current(0.0))
                    }
                }
                Command::Unknown => {
                    println!("Unknown command received");
                    Message::Response(Response::Unknown)
                }
            },
            Message::Response(_) => Message::Response(Response::Unknown),
        }
    }
}
