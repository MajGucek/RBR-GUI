use std::{
    fs::File,
    io::{BufReader, BufRead},
    path::{Path, PathBuf},
};

use sysinfo::{ProcessExt, System, SystemExt};

use crate::common::carFolders::CarFolders;
use crate::common::gearMap::GearMap;

pub struct RBR {
    path: std::path::PathBuf,
    carFolders: CarFolders,
}

const RBR_PROCESS_NAME: &'static str = "RichardBurnsRally_SSE";
impl RBR {
    pub fn find_rbr_process(&mut self) {
        let s = System::new_all();

        for p in s.processes_by_exact_name(RBR_PROCESS_NAME) {
            println!("{}:{}:{}", p.pid(), p.name(), p.root().to_string_lossy());
            self.path = p.root().to_path_buf();
        }
    }

    fn build_physics_path(&self, car_id: i32) -> Option<PathBuf> {
        match self.carFolders.resolve_value(car_id) {
            Some(f) => {
                return Some(self.path.join("Physics").join(f));
            }
            None => {
                return None;
            }
        };
    }

    fn read_rpm_values_from_file(path: PathBuf) -> GearMap {
        let file = File::open(path).unwrap();
    let reader = BufReader::new(file).lines();

    let mut rpms = GearMap {
        gear0_upshift: 0.0,
        gear0_downshift: 0.0,
        gear1_upshift: 0.0,
        gear1_downshift: 0.0,
        gear2_upshift: 0.0,
        gear2_downshift: 0.0,
        gear3_upshift: 0.0,
        gear3_downshift: 0.0,
        gear4_upshift: 0.0,
        gear4_downshift: 0.0,
        gear5_upshift: 0.0,
        gear5_downshift: 0.0,
        gear6_upshift: 0.0,
        gear6_downshift: 0.0,
        rpm_limit: 0.0
    };

    for line in reader {
        let line = line.unwrap();
        if line.contains("Gear") {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            match parts[1] {
                "Gear0Upshift" => rpms.gear0_upshift = parts[2].parse().unwrap(),
                "Gear0Downshift" => rpms.gear0_downshift = parts[2].parse().unwrap(),
                "Gear1Upshift" => rpms.gear1_upshift = parts[2].parse().unwrap(),
                "Gear1Downshift" => rpms.gear1_downshift = parts[2].parse().unwrap(),
                "Gear2Upshift" => rpms.gear2_upshift = parts[2].parse().unwrap(),
                "Gear2Downshift" => rpms.gear2_downshift = parts[2].parse().unwrap(),
                "Gear3Upshift" => rpms.gear3_upshift = parts[2].parse().unwrap(),
                "Gear3Downshift" => rpms.gear3_downshift = parts[2].parse().unwrap(),
                "Gear4Upshift" => rpms.gear4_upshift = parts[2].parse().unwrap(),
                "Gear4Downshift" => rpms.gear4_downshift = parts[2].parse().unwrap(),
                "Gear5Upshift" => rpms.gear5_upshift = parts[2].parse().unwrap(),
                "Gear5Downshift" => rpms.gear5_downshift = parts[2].parse().unwrap(),
                "Gear6Upshift" => rpms.gear6_upshift = parts[2].parse().unwrap(),
                "Gear6DownShift" => rpms.gear6_downshift = parts[2].parse().unwrap(),
                "RPMLimit" => rpms.rpm_limit = parts[2].parse().unwrap(),
                _ => {}
            }
        }
    }
    return rpms;
    }
}
