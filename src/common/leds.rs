use crate::common::rpm::RPM;
use core::u8::{MAX, MIN};
use hidapi::HidDevice;

use super::{rbr::RBR, util::RBR2G29Result};

pub struct LEDS {
    device: HidDevice,
    rpm: RPM,
    state: u8,
    flash_toggled: bool,
    flash_timer: u8,
}

impl LEDS {


    pub fn update(&mut self, data: &[u8]) {
        self.rpm.update(data);
    }


}
