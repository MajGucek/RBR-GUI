use sysinfo::{System, SystemExt, ProcessExt};

pub struct RBR {
    path: std::path::Path,
}
const RBR_PROCESS_NAME : &'static str = "RichardBurnsRally_SSE";
impl RBR {
    

    pub fn find_rbr_process()  {
        let s = System::new_all();

        for p in s.processes_by_exact_name(RBR_PROCESS_NAME) {            
            println!("{}:{}:{}", p.pid(), p.name(), p.root().to_string_lossy());
        }
    }
}


