use egui::{Color32, Pos2, Ui, Rect, Rounding, Sense};
// constants.rs
use crate::constants::*;

pub fn create_line(
    ui: &mut Ui,
    y: f32,
) {
    ui.allocate_ui_at_rect(
            Rect::from_two_pos(
            Pos2::new(0.0, y),
            Pos2::new(PEDAL_WIDTH, y + LINE_SIZE.y)
        ),
        |ui| {
            ui.painter().rect_filled(
                Rect::from_two_pos(
                    Pos2::new(0.0, y),
                    Pos2::new(PEDAL_WIDTH, y + LINE_SIZE.y)
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
    let (response, painter) = ui.allocate_painter(TIRE_SIZE, Sense::hover());
    let c = response.rect.center();
    painter.rect_filled(
        Rect::from_center_size(
            c,
            TIRE_SIZE
        ), 
        Rounding::same(0.0),
        get_color(temperature) 
    );
}

pub fn create_brake(
    ui: &mut Ui,
    temperature: f32,
) {
    let (response, painter) = ui.allocate_painter(BRAKE_SIZE, Sense::hover());
    let c = response.rect.center();
    painter.rect_filled(
        Rect::from_center_size(
            c,
            BRAKE_SIZE
        ), 
        Rounding::same(0.0),
        get_color(temperature) 
    );
}



pub fn get_color(temperature: f32) -> Color32 {
    if temperature > MAX_TEMP {
        return Color32::LIGHT_GREEN;
    }
    if temperature < MIN_TEMP {
        return Color32::DARK_BLUE;
    }
    let temp: u8 = (temperature - 273.15) as u8;
    let g: u8 = 255 - temp;
    let b: u8 = temp;
    Color32::from_rgb(0, g, b)
}