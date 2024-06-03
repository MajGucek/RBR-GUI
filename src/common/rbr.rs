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
            println!("PID: {} exe:{} dir:{}", p.pid(), p.name(), p.root().to_string_lossy());
            self.path = Some(p.root().to_path_buf());
            return Ok(());
        }
        Err(RBR2G29Error::RbrProcessNotFound)
    }
