use rand::Rng;
use socket_udp_protocol::*;
use std::thread;
use std::time::Duration;

use tokio::net::UdpSocket;

#[tokio::main]
async fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8809")
        .await
        .expect("failed to bind host socket");
    println!("starting...");
    loop {
        let temperature: f32 = rand::thread_rng().gen_range(0.0f32..23.0f32);
        let msg = Message { temperature };
        let msg = serialize_message(&msg);
        let send_result = socket.send_to(&msg, "127.0.0.1:8808").await;
        if let Err(err) = send_result {
            println!("can't send temperature: {err}")
        }
        thread::sleep(Duration::from_millis(100))
    }
}
