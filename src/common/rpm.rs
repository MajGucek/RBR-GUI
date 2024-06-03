use crate::common::gear_map::GearMap;
use crate::common::rbr::RBR;
use crate::common::telemetry::Telemetry;
use bincode::deserialize;

#[derive(Default)]
pub struct RPM {
    current: f32,   
    car: i32,
    gear: i32,
    max: f32,
    upshift: f32,
    previous_time: f32,
    gears: GearMap,
}

impl RPM {
    pub fn new() -> Self {
        RPM {
            ..Default::default()
        }
    }

    pub fn update(&mut self, data: &[u8], rbr: &RBR) {
        let telemetry: Telemetry = deserialize(&data).unwrap();
    }
}
