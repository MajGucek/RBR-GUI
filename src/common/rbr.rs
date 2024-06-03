use sysinfo::{ProcessExt, System, SystemExt};


pub struct RBR {
    path: Option<std::path::PathBuf>,
}

const RBR_PROCESS_NAME: &'static str = "RichardBurnsRally_SSE.exe";
impl RBR {
    pub fn new() -> Self {
        RBR {
            path: None,
            car_folders: CarFolders::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.find_rbr_process()?;
    }

    fn find_rbr_process(&mut self) {
        let s = System::new_all();

        for p in s.processes_by_exact_name(RBR_PROCESS_NAME) {
            self.path = Some(p.root().to_path_buf());
        }
    }
}