use socket_udp_protocol::*;
use std::{
    error::Error,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{net::UdpSocket, sync::Mutex, time};

struct SmartThermo {
    temperature: Arc<Mutex<f32>>,
    finished: Arc<AtomicBool>,
}

impl SmartThermo {
    async fn new() -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind("127.0.0.1:8808").await?;
        // socket.set_read_timeout(Some(Duration::from_secs(1)))?;

        let finished = Arc::new(AtomicBool::new(false));
        let temperature = Arc::new(Mutex::new(0f32));

        let temperature_clone = temperature.clone();
        let finished_clone = finished.clone();
        tokio::spawn(async move {
            loop {
                if finished_clone.load(Ordering::SeqCst) {
                    return;
                }
                let mut buf = [0; 1024];
                let read_result = socket.recv_from(&mut buf).await;
                match read_result {
                    Ok(res) => {
                        let msg = deserialize_message(&buf[..res.0]);
                        *temperature_clone.lock().await = msg.temperature;
                    }
                    Err(err) => {
                        println!("can't receive datagram: {err}");
                    }
                }
            }
        });

        Ok(Self {
            temperature,
            finished,
        })
    }
    async fn get_temperature(&mut self) -> f32 {
        *self.temperature.lock().await
    }
}

impl Drop for SmartThermo {
    fn drop(&mut self) {
        self.finished.store(true, Ordering::SeqCst)
    }
}

#[tokio::main]
async fn main() {
    let mut thermo = SmartThermo::new().await.unwrap();
    for _ in 0..120 {
        time::sleep(Duration::from_millis(100)).await;
        let temperature = thermo.get_temperature().await;
        println!("The temperature is {temperature}");
    }
}
