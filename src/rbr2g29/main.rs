use rbr2g29::common::leds::LEDS;
use rbr2g29::common::util::{DR2G27Result, G29_PID, G27_PID, LOGITECH_VID};
use hidapi::{HidApi, HidDevice, DeviceInfo};
use std::net::UdpSocket;
use std::thread::sleep;
use std::time::Duration;

fn read_telemetry_and_update(device: HidDevice) -> DR2G27Result {
    let socket = UdpSocket::bind("127.0.0.1:6776")?;
    let mut leds = LEDS::new(device); 
    let mut data = [0; 664];

    loop {
        match socket.recv(&mut data) {
             Ok(_) => {},
            Err(e) => println!("recv function failed: {e:?}"),
        }
        leds.update(&data)?;
    }
}

fn device_connected(hid: &HidApi) -> Option<DeviceInfo> {
    println!("Looking for devices...");
    for device in hid.device_list() {
        if device.product_id() == G29_PID && device.vendor_id() == LOGITECH_VID && device.interface_number() == 0 {
            println!("Found G29");
            return Some(device.clone());
        }
        if device.product_id() == G27_PID && device.vendor_id() == LOGITECH_VID && device.interface_number() == 0 {
            println!("Found G27");
            return Some(device.clone());
        }
    }

    None
}

fn connect_and_bridge() -> DR2G27Result {  
    println!("Initializing");
    let mut hid = HidApi::new()?;

    match device_connected(&hid) {
        Some(device) =>{
            let dev = device.open_device(&hid)?;                      
            println!("Connected");
            read_telemetry_and_update(dev)?;             
        } ,
        None => println!("Could not find G27 or G29"),
    }
        sleep(Duration::from_secs(1));
        hid.refresh_devices()?;
        Ok(())
}


fn main() {

    loop {
        if let Err(error) = connect_and_bridge() {
            println!("{:?}", error);            
        }

        sleep(Duration::from_secs(1));
    }
}

#[test]
fn test_device_leds() -> DR2G27Result {
    let hid = HidApi::new()?;    

        let Some(devinfo) = hid
        .device_list()
        .filter(|d| d.vendor_id() == LOGITECH_VID && d.product_id() == G29_PID && d.interface_number() == 0)
        .next()        
        else {
            return Err(rbr2g29::common::util::DR2G27Error::G27ConnectionLostError)
        };
            let dev = devinfo.open_device(&hid)?;
   
            for state in vec![0, 1, 3, 7, 15, 31] {
                dev.write(&[ 0x00, 0xF8, 0x12, state, 0x00, 0x00, 0x00, 0x01])?;
                sleep(Duration::from_millis(200));
            }

            sleep(Duration::from_secs(1));

            for state in vec![31, 15, 7, 3, 1, 0] {
                dev.write(&[0x00, 0xF8, 0x12, state, 0x00, 0x00, 0x00, 0x01])?;
                sleep(Duration::from_millis(200));
            }           

    Ok(())
}