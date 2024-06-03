use std::net::UdpSocket;
use bincode::deserialize;
mod telemetry;

fn proccess_telemetry(data: &[u8]) {
    let _telemetry: telemetry::Telemetry = deserialize(&data).unwrap();
}


fn main() {
    loop {
        let socket = UdpSocket::bind("127.0.0.1:6779").expect("failed to bind!");
        let mut data = [0; 664];
        println!("Listening on 127.0.0.1:6779 for telemetry");
        loop {
            match socket.recv(&mut data) {
                Ok(_) => proccess_telemetry(&data),
                Err(e) => println!("recv function failed: {e:?}"),
            };
        }
    }
}
