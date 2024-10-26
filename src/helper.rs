
use egui::{Color32, Pos2, Rect, Rounding, Sense, TextBuffer, Ui};
// constants.rs
use crate::constants::*;
use crate::telemetry::Suspension;

pub fn create_line(
    ui: &mut Ui,
    y: f32,
) {
    ui.allocate_ui_at_rect(
            Rect::from_two_pos(
            Pos2::new(0.0, y),
            Pos2::new(GRAPH_SIZE.x, y + LINE_SIZE.y)
        ),
        |ui| {
            ui.painter().rect_filled(
                Rect::from_two_pos(
                    Pos2::new(0.0, y),
                    Pos2::new(GRAPH_SIZE.x, y + LINE_SIZE.y)
                ),
                Rounding::same(0.0),
                LINE_COLOR 
            );
        });
}

pub fn create_dot(
    ui: &mut Ui,
    x: f32,
    y: f32,
    color: Color32
) {
    ui.allocate_ui_at_rect(
        Rect::from_center_size(
            Pos2::new(x, y),
            DOT_SIZE
        ),
        |ui| {
            ui.painter().circle_filled(
                Pos2::new(x, y), 
                DOT_SIZE.x,
                color 
            );
        });
}

pub fn create_tire(
    ui: &mut Ui,
    temperature: f32,
) {
    //println!("tire temp: {}", temperature);
    let (response, painter) = ui.allocate_painter(TIRE_SIZE, Sense::hover());
    let c = response.rect.center();
    painter.rect_filled(
        Rect::from_center_size(
            c,
            TIRE_SIZE
        ), 
        Rounding::same(0.0),
        get_tire_color(temperature) 
    );
}

pub fn create_brake(
    ui: &mut Ui,
    temperature: f32,
) {
    //println!("brake temp: {}", temperature);
    let (response, painter) = ui.allocate_painter(BRAKE_SIZE, Sense::hover());
    let c = response.rect.center();
    painter.rect_filled(
        Rect::from_center_size(
            c,
            BRAKE_SIZE
        ), 
        Rounding::same(0.0),
        get_brake_color(temperature) 
    );
}

pub fn create_suspension(
    ui: &mut Ui,
    spring: Suspension,
) {
    let (response, painter) = ui.allocate_painter(SUSPENSION_SIZE, Sense::hover());
    let c = response.rect.center();
    painter.rect_filled(
        Rect::from_center_size(
            c,
            SUSPENSION_SIZE
        ), 
        Rounding::same(0.0),
        Color32::RED
    );
}



pub fn get_tire_color(temperature: f32) -> Color32 {
    if temperature > MAX_TIRE_TEMP {
        return Color32::LIGHT_GREEN;
    }
    if temperature < MIN_TIRE_TEMP {
        return Color32::DARK_BLUE;
    }
    let b: u8 = 255 - (temperature * 3.0) as u8;
    let g: u8 = (temperature * 3.0) as u8;
    Color32::from_rgb(0, g, b)
}

fn get_brake_color(temperature: f32) -> Color32 {
    if temperature > MAX_BRAKE_TEMP {
        return Color32::LIGHT_GREEN;
    }
    if temperature < MIN_BRAKE_TEMP {
        return Color32::DARK_BLUE;
    }
    let b: u8 = 255 - (temperature / 2.0) as u8;
    let g: u8 = (temperature / 2.0) as u8;
    Color32::from_rgb(0, g, b)
}



pub fn format_time(minutes: f32, seconds: f32) -> String {
    let mut time: String = String::new(); 
    if minutes < 10.0 {
        time.push_str(format!("0{}", minutes).as_str());
    } else {
        time.push_str(format!("{}", minutes).as_str());
    }
    time.push_str(":".as_str());
    if seconds < 10.0 {
        time.push_str(format!("0{}", seconds).as_str());
    } else {
        time.push_str(format!("{}", seconds).as_str());
    }
    let remainder = (seconds.fract() * 100.0).round();
    if remainder == 0.0 {
        time.push_str(".00".as_str());
    } else if remainder % 10.0 == 0.0 {
        time.push_str("0".as_str());
    }
    time
}