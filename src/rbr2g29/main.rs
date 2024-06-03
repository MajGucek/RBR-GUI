use std::net::UdpSocket;
use std::thread::sleep;
use std::time::Duration;

fn update(data: &[u8]) {
    let telemetry: Telemetry = deserialize(&data).unwrap();
}

fn read_telemetry_and_update(device: HidDevice, ip: &String, port: &String) -> RBR2G29Result {   
    let mut rbr = rbr2g29::common::rbr::RBR::new();

    println!("Looking for RBR process...");
    loop {
        match rbr.initialize() {
            Err(_) => {
                sleep(Duration::from_secs(1));
            }
            _ => break,
        }
    }
    let socket = UdpSocket::bind("127.0.0.1:6779")?;
    let mut data = [0; 664];
    println!("Listening on 127.0.0.1:6779 for telemetry");
    loop {
        match socket.recv(&mut data) {
            Ok(_) => update(&data)?,
            Err(e) => println!("recv function failed: {e:?}"),
        };
    }
}

fn main() {
    loop {
        read_telemetry_and_update();
    }
}
