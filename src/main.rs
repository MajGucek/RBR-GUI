use std::net::UdpSocket;
use rbrgui::common::telemetry::Telemetry;

fn update(data: &[u8]) {
    let telemetry: Telemetry = deserialize(&data).unwrap();
}

fn read_telemetry_and_update() {   
    
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
