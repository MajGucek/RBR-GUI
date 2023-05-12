use std::io::Error;
use hidapi::HidError;


pub const LOGITECH_VID: u16 = 1133; // Vendor ID
pub const G29_PID: u16 = 49743; // Product ID
pub const G27_PID: u16 = 49819; // Product ID

pub type RBR2G29Result = Result<(), RBR2G29Error>;

#[derive(Debug)]
pub enum RBR2G29Error {
    UdpConnectionError,
    DeviceConnectionLostError,
}

impl From<Error> for RBR2G29Error {
    fn from(_: Error) -> Self {
        Self::UdpConnectionError
    }
}

impl From<HidError> for RBR2G29Error {
    fn from(_: HidError) -> Self {
        Self::DeviceConnectionLostError
    }
}