use serde::Deserialize;

// Telemetry data is ported from the NGP rbr.telemetry.data.TelemetryData.h header
// that is part of the NGP plugin.

#[derive(Deserialize, Default)]
pub struct TireSegment {
    pub temperature: f32,
    pub wear: f32,
}

#[derive(Deserialize, Default)]
pub struct Tire {
    pub pressure: f32,
    pub temperature: f32,
    pub carcass_temperature: f32,
    pub tread_temperature: f32,
    pub current_segment: u32,
    pub segment1: TireSegment,
    pub segment2: TireSegment,
    pub segment3: TireSegment,
    pub segment4: TireSegment,
    pub segment5: TireSegment,
    pub segment6: TireSegment,
    pub segment7: TireSegment,
    pub segment8: TireSegment,
}

#[derive(Deserialize, Default)]
pub struct BrakeDisk {
    pub layer_temperature: f32,
    pub temperature: f32,
    pub wear: f32,
}

#[derive(Deserialize, Default)]
pub struct Wheel {
    pub brake_disk: BrakeDisk,
    pub tire: Tire,
}

#[derive(Deserialize, Default)]
pub struct Damper {
    pub damage: f32,
    pub piston_velocity: f32,
}

#[derive(Deserialize, Default)]
pub struct Suspension {
    pub spring_deflection: f32,
    pub rollbar_force: f32,
    pub spring_force: f32,
    pub damper_force: f32,
    pub strut_force: f32,
    pub helper_spring_is_active: i32,
    pub damper: Damper,
    pub wheel: Wheel,
}

#[derive(Deserialize, Default)]
pub struct Engine {
    pub rpm: f32,
    pub radiator_coolant_temperature: f32,
    pub engine_coolant_temperature: f32,
    pub engine_temperature: f32,
}

#[derive(Deserialize, Default)]
pub struct Motion {
    pub surge: f32,
    pub sway: f32,
    pub heave: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

#[derive(Deserialize, Default)]
pub struct Car {
    pub index: i32,
    pub speed: f32,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocities: Motion,
    pub accelerations: Motion,
    pub engine: Engine,
    pub suspension_lf: Suspension,
    pub suspension_rf: Suspension,
    pub suspension_lb: Suspension,
    pub suspension_rb: Suspension,
}

#[derive(Deserialize, Default)]
pub struct Control {
    pub steering: f32,
    pub throttle: f32,
    pub brake: f32,
    pub handbrake: f32,
    pub clutch: f32,
    pub gear: i32,
    pub footbrake_pressure: f32,
    pub handbrake_pressure: f32,
}

#[derive(Deserialize, Default)]
pub struct Stage {
    pub index: i32,
    pub progress: f32,
    pub race_time: f32, // seconds
    pub drive_line_location: f32,
    pub distance_to_end: f32,
}

#[derive(Deserialize, Default)]
pub struct Telemetry {
    pub total_steps: u32,
    pub stage: Stage,
    pub control: Control,
    pub car: Car,
}

impl Telemetry {
    pub fn format(&mut self) {
        self.control.brake *= 100.0;
        self.control.throttle *= 100.0;
        self.control.clutch *= 100.0;
        self.control.gear -= 1;
    }
    pub fn get_time(&self) -> (f32, f32, f32) {
        let race_time: &f32 = &self.stage.race_time;
        let hr: f32 = (race_time / 3600.0).round();
        let min: f32 = (race_time / 60.0).round();
        let sec: f32 = f32::trunc(race_time * 1000.0) / 1000.0;
        (sec, min, hr)
    }
}
