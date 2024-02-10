use socket_tcp_protocol::*;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::Mutex,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8809").await.expect("can't bind tcp listener");

    let mut smart_socket = SmartSocket::default();


    while let Some(connection) = listener.incoming().next() {
        let mut stream = match connection {
            Ok(conn) => conn,
            Err(err) => {
                println!("can't receive connection: {err}");
                continue;
            }
        };

        let peer = stream
            .peer_addr()
            .map(|a| a.to_string())
            .unwrap_or_else(|_| "unknown".into());
        println!("Peer '{peer}' connected");

        let mut in_size_buffer = [0u8; 4];
        while stream.read_exact(&mut in_size_buffer).is_ok() {
            let size: usize = u32::from_be_bytes(in_size_buffer) as usize;
            let mut payload = vec![0; size];
            stream.read_exact(&mut payload[..]).unwrap();

            let msg = deserialize_message(payload.as_slice());

            let response = smart_socket.process_message(&msg);
            println!("process messgae: {msg} -> {response}");
            let response_buf = serialize_message(&response);
            let response_size: u32 = response_buf.len() as u32;
            let size_slice: [u8; 4] = response_size.to_be().to_ne_bytes();
            if stream.write_all(&size_slice).is_err() {
                break;
            };
            if stream.write_all(&response_buf).is_err() {
                break;
            };
        }

        println!("Connection with {peer} lost. Waiting for new connections...");
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
