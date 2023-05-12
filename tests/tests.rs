#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use hidapi::HidApi;
    use rbr2g29::common::{
        rbr::RBR,
        util::{RBR2G29Result, G29_PID, LOGITECH_VID},
    };

    #[test]
    fn test_device_leds() -> RBR2G29Result {
        let hid = HidApi::new()?;

        let Some(devinfo) = hid
            .device_list()
            .filter(|d| d.vendor_id() == LOGITECH_VID && d.product_id() == G29_PID && d.interface_number() == 0)
            .next()
            else {
                return Err(rbr2g29::common::util::RBR2G29Error::DeviceConnectionLostError)
            };
        let dev = devinfo.open_device(&hid)?;

        for state in vec![0, 1, 3, 7, 15, 31] {
            dev.write(&[0x00, 0xF8, 0x12, state, 0x00, 0x00, 0x00, 0x01])?;
            sleep(Duration::from_millis(200));
        }

        sleep(Duration::from_secs(1));

        for state in vec![31, 15, 7, 3, 1, 0] {
            dev.write(&[0x00, 0xF8, 0x12, state, 0x00, 0x00, 0x00, 0x01])?;
            sleep(Duration::from_millis(200));
        }

        Ok(())
    }
   
}
