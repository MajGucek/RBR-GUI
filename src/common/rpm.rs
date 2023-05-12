use crate::common::telemetry::Telemetry;
use bincode::deserialize;

#[derive(Default)]
pub struct RPM {
    current: f32,
    idle: f32,
    car: i32,
    gear: i32,
    max: f32,
}



impl RPM {
    pub fn new() -> Self {
        RPM {
            car: -1,
            gear: -1,
            max: 5000.0,
            ..Default::default()
        }
    }

    pub fn state(&self) -> (f32, f32, f32) {
        (self.current, self.max * 0.95, self.idle)
    }

    pub fn update(&mut self, data: &[u8]) {
        let telemetry: Telemetry = deserialize(&data).unwrap();
        let current_rpm = telemetry.car.engine.rpm;

        if self.car != telemetry.car.index {
            println!("Car change detected");
            self.car = telemetry.car.index;
            self.max = 5000.0;
        }
        if self.gear != telemetry.control.gear {
            self.gear = telemetry.control.gear;
            self.max = 5000.0;
        }

        if (self.current) != current_rpm {
            self.current = current_rpm;
            self.idle = 1000.0;
            if self.max <= current_rpm {
                self.max = current_rpm;
            }
        }
    }
}
