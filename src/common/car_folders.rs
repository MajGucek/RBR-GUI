use std::collections::HashMap;

pub struct CarFolders {
    values_lookup: HashMap<i32, String>,
}

impl CarFolders {

    pub fn new() -> Self {
        let mut values_lookup = HashMap::new();
        values_lookup.insert(0, "c_xsara".to_string());
        values_lookup.insert(1, "h_accent".to_string());
        values_lookup.insert(2, "mg_zr".to_string());
        values_lookup.insert(3, "m_lancer".to_string());
        values_lookup.insert(4, "p_206".to_string());
        values_lookup.insert(5, "s_i2003".to_string());
        values_lookup.insert(6, "t_coroll".to_string());
        values_lookup.insert(7, "s_i2000".to_string());

        CarFolders { values_lookup }
    }
    
    pub fn resolve_value(&self, index: i32) -> Option<String>{
        self.values_lookup.get(&index).cloned()
    }
}