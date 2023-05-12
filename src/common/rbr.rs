use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use sysinfo::{ProcessExt, System, SystemExt};

use crate::common::carFolders::CarFolders;
use crate::common::gearMap::GearMap;

use crate::common::util::{RBR2G29Error, RBR2G29Result};

pub struct RBR {
    path: Option<std::path::PathBuf>,
    carFolders: CarFolders,
}

const RBR_PROCESS_NAME: &'static str = "RichardBurnsRally_SSE.exe";
impl RBR {
    pub fn new() -> Self {
        RBR {
            path: None,
            carFolders: CarFolders::new(),
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
            if let Some(f) = self.carFolders.resolve_value(car_id) {
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
                let lineContents = line.split_whitespace();
                let parts = lineContents.collect::<Vec<&str>>();                

                if(parts.len() != 2){
                    continue;
                } 

                let gearIdentifier = parts[0].trim();
                let rpmValue: f32 = parts[1].trim().parse().unwrap();
                match gearIdentifier {                    
                    "gear0upshift" => rpms.gear0_upshift = rpmValue,
                    "gear0downshift" => rpms.gear0_downshift = rpmValue,
                    "gear1upshift" => rpms.gear1_upshift = rpmValue,
                    "gear1downshift" => rpms.gear1_downshift = rpmValue,
                    "gear2upshift" => rpms.gear2_upshift = rpmValue,
                    "gear2downshift" => rpms.gear2_downshift = rpmValue,
                    "gear3upshift" => rpms.gear3_upshift = rpmValue,
                    "gear3downshift" => rpms.gear3_downshift = rpmValue,
                    "gear4upshift" => rpms.gear4_upshift = rpmValue,
                    "gear4downshift" => rpms.gear4_downshift = rpmValue,
                    "gear5upshift" => rpms.gear5_upshift = rpmValue,
                    "gear5downshift" => rpms.gear5_downshift = rpmValue,
                    "gear6upshift" => rpms.gear6_upshift = rpmValue,
                    "gear6downshift" => rpms.gear6_downshift = rpmValue,
                    "rpmlimit" => rpms.rpm_limit = rpmValue,
                    
                    _ => {}
                }
            }
        }
        return rpms;
    }
}
