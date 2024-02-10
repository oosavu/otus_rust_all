use socket_udp_protocol::*;
use std::{
    error::Error,
    net::UdpSocket,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

struct SmartThermo {
    temperature: Arc<Mutex<f32>>,
    finished: Arc<AtomicBool>,
}

impl SmartThermo {
    fn new() -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind("127.0.0.1:8808")?;
        socket.set_read_timeout(Some(Duration::from_secs(1)))?;

        let finished = Arc::new(AtomicBool::new(false));
        let temperature = Arc::new(Mutex::new(0f32));

        let temperature_clone = temperature.clone();
        let finished_clone = finished.clone();
        thread::spawn(move || loop {
            if finished_clone.load(Ordering::SeqCst) {
                return;
            }
            let mut buf = [0; 1024];
            let read_result = socket.recv_from(&mut buf);
            match read_result {
                Ok(res) => {
                    let msg = deserialize_message(&buf[..res.0]);
                    *temperature_clone.lock().unwrap() = msg.temperature;
                }
                Err(err) => {
                    println!("can't receive datagram: {err}");
                }
            }
        });

        Ok(Self {
            temperature,
            finished,
        })
    }
    fn get_temperature(&mut self) -> f32 {
        *self.temperature.lock().unwrap()
    }
}

impl Drop for SmartThermo {
    fn drop(&mut self) {
        self.finished.store(true, Ordering::SeqCst)
    }
}

fn main() {
    let mut thermo = SmartThermo::new().unwrap();
    for _ in 0..120 {
        thread::sleep(Duration::from_secs(1));
        let temperature = thermo.get_temperature();
        println!("The temperature is {temperature}");
    }
}
