use egui::{Vec2, Color32, Pos2};

pub const UDP_IP: &str = "127.0.0.1:";
pub const WIDTH: f32 = 400.0;
pub const HEIGHT: f32 = 400.0;
pub const ZERO: Pos2 = Pos2::new(0.0, 0.0);
pub const HORIZONTAL_CENTER: f32 = 50.0;
pub const VERTICAL_CENTER: f32 = 50.0;
pub const TIRE_SIZE: Vec2 = Vec2::splat(100.0);
pub const BRAKE_SIZE: Vec2 = Vec2::new(25.0, 75.0);
pub const SUSPENSION_SIZE: Vec2 = Vec2::new(50.0, 100.0);
pub const GRAPH_SIZE: Vec2 = Vec2::new(600.0, 200.0);
pub const DOT_SPACING: f32 = 0.75;
pub const DOT_SIZE: Vec2 = Vec2::splat(DOT_SPACING);
pub const LINE_SIZE: Vec2 = Vec2::new(600.0, DOT_SIZE.y / 2.0);
pub const SPACING: f32 = 50.0;
pub const CHECKBOX_SPACING: f32 = 10.0;
pub const WORD_SPACING: f32 = 70.0;
pub const BRAKE_SPACING: f32 = 5.0;
pub const BRAKE_VERTICAL_SPACING: f32 = 10.0;
pub const TIRE_HORIZONTAL_SPACING: f32 = 30.0;

pub const MIN_TIRE_TEMP: f32 = 30.0;
pub const MAX_TIRE_TEMP: f32 = 75.0;

pub const MIN_BRAKE_TEMP: f32 = 180.0;
pub const MAX_BRAKE_TEMP: f32 = 360.0;

pub const MENU_BG: Color32 = Color32::from_rgb(32,32,32);
pub const LINE_COLOR: Color32 = Color32::GRAY;