pub struct GearMap {
   pub gear0_upshift: f32,
   pub gear0_downshift: f32,
   pub gear1_upshift: f32,
   pub gear1_downshift: f32,
   pub gear2_upshift: f32,
   pub gear2_downshift: f32,
   pub gear3_upshift: f32,
   pub gear3_downshift: f32,
   pub gear4_upshift: f32,
   pub gear4_downshift: f32,
   pub gear5_upshift: f32,
   pub gear5_downshift: f32,
   pub gear6_upshift: f32,
   pub gear6_downshift: f32,
   pub rpm_limit: f32,
}

impl Default for GearMap {
    fn default () -> GearMap {
        GearMap{gear0_upshift: 0.0, gear0_downshift: 0.0, gear1_upshift: 0.0, gear1_downshift: 0.0, gear2_upshift: 0.0, gear2_downshift: 0.0, gear3_upshift: 0.0, gear3_downshift: 0.0, gear4_upshift: 0.0, gear4_downshift: 0.0, gear5_upshift: 0.0, gear5_downshift: 0.0, gear6_upshift: 0.0, gear6_downshift: 0.0, rpm_limit: 0.0 }
    }
}

impl GearMap{

    pub fn get_rpm_for_gear(&self, gear:i32) -> f32  {

        // R = 0
        // N = 1    
        match gear {
            0 => self.gear0_upshift,
            1 => self.gear1_upshift,
            2 => self.gear2_upshift,
            3 => self.gear3_upshift,
            4 => self.gear4_upshift,
            5 => self.gear5_upshift,
            6 => self.gear6_upshift,
            _ => 0.0
        }
    }
  
}