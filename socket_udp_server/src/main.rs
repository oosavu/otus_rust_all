use std::net::{ToSocketAddrs, UdpSocket};
use socket_udp_protocol::*;
use std::thread;
use std::time::{Duration, Instant};
use rand;
use rand::Rng;

fn main() {
    let host_addr = "localhost:8809".to_socket_addrs().unwrap();
    let client_addr = "localhost:8808".to_socket_addrs().unwrap();
    let socket = UdpSocket::bind(host_addr).expect("failed to bind host socket");
    println!("starting...");
    loop {
        let temperature: f32 = rand::thread_rng().gen_range(0.0f32..23.0f32);
        let msg = serialize_message(&Message::Temperature(temperature));
        let size: u32 = msg.len() as u32;
        let mut size_slice: [u8; 4] = size.to_be_bytes();
        let data = size_slice.into_iter().chain(msg.into_iter()).collect();
        let send_result = socket.send_to(&data, client_addr.clone());
        if let Err(err) = send_result {
            println!("can't send temperature: {err}")
        }
        thread::sleep(Duration::from_secs(1))
    }
}