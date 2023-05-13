use crate::common::telemetry::Telemetry;
use crate::common::rbr::RBR;
use bincode::deserialize;

use super::gearMap::{self, GearMap};

#[derive(Default)]
pub struct RPM {
    current: f32,
    idle: f32,
    car: i32,
    gear: i32,
    max: f32,
    upshift: f32,
    gears: GearMap,    
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
        (self.current, self.max, self.idle)
    }

    pub fn update(&mut self, data: &[u8], rbr: &RBR) {
        let telemetry: Telemetry = deserialize(&data).unwrap();
        let current_rpm = telemetry.car.engine.rpm;

        if self.car != telemetry.car.index || telemetry.stage.race_time == 0.0 {
            println!("Car change or new stage detected, updating RPM maps");
            self.car = telemetry.car.index;
            let path = rbr.build_physics_path(self.car);
            if let Some(p) = path{
                self.gears = RBR::read_rpm_values_from_file(p);                
            }
        }
        if self.gear != telemetry.control.gear {                  
            self.gear = telemetry.control.gear;
            self.upshift = self.gears.get_rpm_for_gear(self.gear);            
        }

        if (self.current) != current_rpm {
            self.current = current_rpm;          
        }
        self.max = self.gears.rpm_limit;
    }
}
