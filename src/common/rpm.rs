use std::path::{PathBuf};
use std::{fs, io};

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
            car: -1,
            gear: -1,
            max: 5000.0,
            ..Default::default()
        }
    }

    pub fn state(&self) -> (f32, f32) {
        (self.current, self.max)
    }

    fn get_car_name_from_path(path: &PathBuf) -> Result<Option<PathBuf>, io::Error> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let p = entry.path();
            if p.is_file() {
                if entry.path().extension().is_none() {                    
                    return Ok(Some(entry.path()));
                }
            }
        }
        return Ok(None);
    }

    pub fn update(&mut self, data: &[u8], rbr: &RBR) {
        let telemetry: Telemetry = deserialize(&data).unwrap();
        let current_rpm = telemetry.car.engine.rpm;

        if self.car != telemetry.car.index || (telemetry.stage.race_time == 0.0 && self.previous_time > 0.0) {
            println!("Car change or new stage detected, updating RPM maps");
            self.previous_time = 0.0;
            self.car = telemetry.car.index;
            let path = rbr.build_physics_path(self.car);
            if let Some(p) = path {
                let car = Self::get_car_name_from_path(&p);
                
                match car.unwrap() {
                    Some(c) => println!("Looks like we're driving the {}", c.file_name().unwrap().to_str().unwrap()),
                    None => println!("Unable to determine car name"),
                }
                self.gears = RBR::read_rpm_values_from_file(p);
            }
        }
        self.previous_time = telemetry.stage.race_time;
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
