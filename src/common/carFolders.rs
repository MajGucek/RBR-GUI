use std::collections::BTreeMap;

struct CarFolders {
    values_lookup: HashMap<i32, &str>,
}

impl CarFolders {

    pub fn new() -> Self {
        let mut values_lookup = HashMap::new();
        values_lookup.insert(0, "c_xsara");
        values_lookup.insert(1, "h_accent");
        values_lookup.insert(2, "mg_zr");
        values_lookup.insert(3, "m_lancer");
        values_lookup.insert(4, "p_206");
        values_lookup.insert(5, "s_i2003");
        values_lookup.insert(6, "t_coroll");
        values_lookup.insert(7, "s_i2000");

        CarFolders { values_lookup }
    }
    
    pub fn resolve_value(index: i32) -> str{
        values_lookup.get(&index).cloned()
    }
}