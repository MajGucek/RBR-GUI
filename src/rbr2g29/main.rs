use clap::{arg, command};
use hidapi::{DeviceInfo, HidApi, HidDevice};
use rbr2g29::common::leds::LEDS;
use rbr2g29::common::util::{RBR2G29Result, G27_PID, G29_PID, G920_PID, LOGITECH_VID};
use std::net::UdpSocket;
use std::thread::sleep;
use std::time::Duration;


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
            Ok(_) => leds.update(&data)?,
            Err(e) => println!("recv function failed: {e:?}"),
        };
    }
}

fn main() {
    loop {
        read_telemetry_and_update();
    }
}
