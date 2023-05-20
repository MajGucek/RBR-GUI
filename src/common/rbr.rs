use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{PathBuf},
};

use sysinfo::{ProcessExt, System, SystemExt};

use crate::common::car_folders::CarFolders;
use crate::common::gear_map::GearMap;

use crate::common::util::{RBR2G29Error, RBR2G29Result};

pub struct RBR {
    path: Option<std::path::PathBuf>,
    car_folders: CarFolders,
}

const RBR_PROCESS_NAME: &'static str = "RichardBurnsRally_SSE.exe";
impl RBR {
    pub fn new() -> Self {
        RBR {
            path: None,
            car_folders: CarFolders::new(),
        }
    }

    pub fn initialize(&mut self) -> RBR2G29Result {
        self.find_rbr_process()?;
        Ok(())
    }

    fn find_rbr_process(&mut self) -> RBR2G29Result {
        let s = System::new_all();

        for p in s.processes_by_exact_name(RBR_PROCESS_NAME) {
            println!("{}:{}:{}", p.pid(), p.name(), p.root().to_string_lossy());
            self.path = Some(p.root().to_path_buf());
            return Ok(());
        }
        Err(RBR2G29Error::RbrProcessNotFound)
    }

    pub fn build_physics_path(&self, car_id: i32) -> Option<PathBuf> {
        if let Some(p) = &self.path {
            if let Some(f) = self.car_folders.resolve_value(car_id) {
                return Some(p.join("Physics").join(f));
            }
        }

        return None;
    }

    pub fn read_rpm_values_from_file(path: PathBuf) -> GearMap {
        let file = File::open(path.join("common.lsp")).unwrap();
        let reader = BufReader::new(file).lines();

        let mut rpms = GearMap::default();

        for line in reader {
            let line = line.unwrap().to_lowercase();
            if line.contains("gear") | line.contains("rpmlimit") {
                let line_contents = line.split_whitespace();
                let parts = line_contents.collect::<Vec<&str>>();                

                if parts.len() != 2 {
                    continue;
                } 

                let gear_identifier = parts[0].trim();
                let rpm_value: f32 = parts[1].trim().parse().unwrap();
                match gear_identifier {                    
                    "gear0upshift" => rpms.gear0_upshift = rpm_value,
                    "gear0downshift" => rpms.gear0_downshift = rpm_value,
                    "gear1upshift" => rpms.gear1_upshift = rpm_value,
                    "gear1downshift" => rpms.gear1_downshift = rpm_value,
                    "gear2upshift" => rpms.gear2_upshift = rpm_value,
                    "gear2downshift" => rpms.gear2_downshift = rpm_value,
                    "gear3upshift" => rpms.gear3_upshift = rpm_value,
                    "gear3downshift" => rpms.gear3_downshift = rpm_value,
                    "gear4upshift" => rpms.gear4_upshift = rpm_value,
                    "gear4downshift" => rpms.gear4_downshift = rpm_value,
                    "gear5upshift" => rpms.gear5_upshift = rpm_value,
                    "gear5downshift" => rpms.gear5_downshift = rpm_value,
                    "gear6upshift" => rpms.gear6_upshift = rpm_value,
                    "gear6downshift" => rpms.gear6_downshift = rpm_value,
                    "rpmlimit" => rpms.rpm_limit = rpm_value,
                    
                    _ => {}
                }
            }
        }
        return rpms;
    }
}
