#![allow(dead_code)]
use serde::Deserialize;

// Telemetry data is ported from the NGP rbr.telemetry.data.TelemetryData.h header
// that is part of the NGP plugin.

#[derive(Deserialize, Default)]
pub struct Stage {
    index: i32,
    progress: f32,
    race_time: f32,
    drive_line_location: f32,
    distance_to_end: f32,
}

#[derive(Deserialize, Default)]
pub struct Engine {
    rpm: f32,
    radiator_coolant_temperature: f32,
    engine_coolant_temperature: f32,
    engine_temperature: f32,
}

#[derive(Deserialize, Default)]
pub struct Motion {
    surge: f32,
    sway: f32,
    heave: f32,
    roll: f32,
    pitch: f32,
    yaw: f32,
}

#[derive(Deserialize, Default)]
pub struct Car {
    index: i32,
    speed: f32,
    position_x: f32,
    position_y: f32,
    position_z: f32,
    roll: f32,
    pitch: f32,
    yaw: f32,
    velocities: Motion,
    accelerations: Motion,
    engine: Engine,
}

#[derive(Deserialize, Default)]
pub struct Control {
    steering: f32,
    throttle: f32,
    brake: f32,
    handbrake: f32,
    clutch: f32,
    gear: i32,
    footbrake_pressure: f32,
    handbrake_pressure: f32,
}

#[derive(Deserialize, Default)]
pub struct TireSegment {
    temperature: f32,
    wear: f32,
}

#[derive(Deserialize, Default)]
pub struct Tire {
    pressure: f32,
    temperature: f32,
    carcass_temperature: f32,
    tread_temperature: f32,
    current_segment: u32,
    segment1: TireSegment,
    segment2: TireSegment,
    segment3: TireSegment,
    segment4: TireSegment,
    segment5: TireSegment,
    segment6: TireSegment,
    segment7: TireSegment,
    segment8: TireSegment,
}

#[derive(Deserialize, Default)]
pub struct BrakeDisk {
    layer_temperature: f32,
    temperature: f32,
    wear: f32,
}

#[derive(Deserialize, Default)]
pub struct Wheel {
    brake_disk: BrakeDisk,
    tire: Tire,
}

#[derive(Deserialize, Default)]
pub struct Damper {
    damage: f32,
    piston_velocity: f32,
}

#[derive(Deserialize, Default)]
pub struct Suspension {
    spring_deflection: f32,
    rollbar_force: f32,
    spring_force: f32,
    damper_force: f32,
    strut_force: f32,
    helper_spring_is_active: i32,
    damper: Damper,
    wheel: Wheel,
}

#[derive(Deserialize, Default)]
pub struct Telemetry {
    total_steps: u32,
    stage: Stage,
    control: Control,
    car: Car,
}


