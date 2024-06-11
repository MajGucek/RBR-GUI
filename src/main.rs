// Hide Terminal
//#![windows_subsystem = "windows"]

// Remove and fix before release
#![allow(dead_code, 
    unused_variables, 
    unused_mut, 
    unused_imports, 
    unused_parens
)]

use std::net::UdpSocket;
use std::io::Error;
use std::collections::VecDeque;
//
use bincode::deserialize;
// telemetry.rs
mod telemetry;
use telemetry::{Control, Telemetry};

// UI
use bevy::{prelude::*, window::WindowLevel, window::Cursor};
use bevy::time::common_conditions::on_timer;
use bevy::utils::Duration;
use bevy_egui::{EguiContexts, EguiPlugin};
use egui::{Color32, FontId, Frame, Margin, Pos2, Rect, Rounding, Sense, TextBuffer, Ui, Vec2};


const UDP_IP: &str = "127.0.0.1:";
const WIDTH: f32 = 400.0;
const HEIGHT: f32 = 400.0;
const ZERO: Pos2 = Pos2::new(0.0, 0.0);
const HORIZONTAL_CENTER: f32 = 70.0;
const VERTICAL_CENTER: f32 = 50.0;
const SPACING: f32 = 50.0;
const TIRE_SIZE: Vec2 = Vec2::splat(100.0);
const BRAKE_SIZE: Vec2 = Vec2::new(25.0, 75.0);
const GRAPH_SIZE: Vec2 = Vec2::new(WIDTH * 2.0, 200.0);
const DOT_SIZE: Vec2 = Vec2::splat(1.0);
const DELTA_SIZE: Vec2 = Vec2::new(350.0, 50.0);
const DOT_SPACING: u32 = 1;
const CHECKBOX_SPACING: f32 = 10.0;
const BRAKE_SPACING: f32 = 5.0;
const BRAKE_VERTICAL_SPACING: f32 = 10.0;
const TIRE_HORIZONTAL_SPACING: f32 = 30.0;
const MAX_TEMP: f32 = 150.0 + 273.15;
const MIN_TEMP: f32 = 40.0 + 273.15;

#[derive(Resource)]
struct RBR {
    telemetry: Telemetry,
    recv: bool,
}
impl RBR {
    fn get_data(&mut self, data: &[u8]) {
        self.telemetry = deserialize(&data).unwrap();
        self.telemetry.format();
    }
}
impl Default for RBR {
    fn default() -> Self {
        RBR {
            telemetry: Telemetry::default(),
            recv: false,
        }
    }
}



#[derive(Resource)]
struct Socket {
    socket: Result<UdpSocket, Error>,
    address: String,
}
impl Socket {
    fn bind(&mut self, port: &str) {
        self.address = format!("{UDP_IP}{port}");
        self.socket = UdpSocket::bind(&self.address);
    }
}

impl Default for Socket {
    fn default() -> Self {
        Socket {
            address: String::new(),
            socket: UdpSocket::bind(String::new()),
        }
    }
}

#[derive(Resource)]
struct Port {
    port: String,
}

impl Default for Port {
    fn default() -> Self {
        Port {
            port: String::new(),
        }
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum ConnectionState {
    Disconnected,
    Connected,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum DisplayState {
    Main,
    Tyres,
    Pedals,
    Delta,
}


#[derive(Resource)]
struct Pedals {
    throttle: VecDeque<f32>,
    brake: VecDeque<f32>,
    handbrake: VecDeque<f32>,
    clutch: VecDeque<f32>,
    gear: VecDeque<i32>,
    size: u32,
}
impl Pedals {
    fn add_data(&mut self, data: &Control) {
        if self.size > (GRAPH_SIZE.x as u32) {
            self.throttle.pop_front();
            self.brake.pop_front();
            self.handbrake.pop_front();
            self.clutch.pop_front();
            self.gear.pop_front();
        } else {
            self.size += 1;
        }
        self.throttle.push_back(data.throttle);
        self.brake.push_back(data.brake);
        self.handbrake.push_back(data.handbrake);
        self.clutch.push_back(data.clutch);
        self.gear.push_back(data.gear);
    }
}
impl Default for Pedals {
    fn default() -> Self {
        Pedals {
            throttle: VecDeque::new(),
            brake: VecDeque::new(),
            clutch: VecDeque::new(),
            gear: VecDeque::new(),
            handbrake: VecDeque::new(),
            size: 0,
        }
    }
}


#[derive(Resource)]
struct PedalCheckboxes {
    throttle: bool,
    brake: bool, 
    handbrake: bool,
    clutch: bool,
    gear: bool,
}
impl Default for PedalCheckboxes {
    fn default() -> Self {
        PedalCheckboxes {
            throttle: true,
            brake: true,
            handbrake: false,
            clutch: false,
            gear: false,
        }
    }
}


fn main() {
    let window = Window {
        window_level: WindowLevel::AlwaysOnTop,
        ..default()
    };
    App::new()
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(window),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .insert_state(DisplayState::Main)
        .insert_state(ConnectionState::Disconnected)
        .init_resource::<Socket>()
        .init_resource::<RBR>()
        .init_resource::<Port>()
        .init_resource::<Pedals>()
        .init_resource::<PedalCheckboxes>()
        .add_systems(
            Update,
            (   
                
                telemetry_handler
                    .run_if(in_state(ConnectionState::Connected)),
                connect_udp
                    .run_if(in_state(ConnectionState::Disconnected))
                    .run_if(on_timer(Duration::from_secs(2)))
                    )
        )
        
        .add_systems(Update, 
            (
                main_menu.run_if(in_state(DisplayState::Main)),
                pedal_menu.run_if(in_state(DisplayState::Pedals)),
                tire_menu.run_if(in_state(DisplayState::Tyres)),
                delta_menu.run_if(in_state(DisplayState::Delta)),
        )
    )
    .run();
}

fn delta_menu(
    mut windows: Query<&mut Window>,
    mut egui_ctx: EguiContexts,
    mut next_state: ResMut<NextState<DisplayState>>,
    rbr: Res<RBR>,
) {
    let mut window = windows.single_mut();
    window.resolution.set(WIDTH, HEIGHT / 2.0);
    let gui = egui::Window::new("gui")
        .title_bar(false)
        .fixed_pos(ZERO)
        .default_height(HEIGHT)
        .default_width(WIDTH)
        .collapsible(false)
        .resizable(false)
        .frame(Frame {
            inner_margin: Margin::same(0.0),
            outer_margin: Margin::same(0.0),
            ..default()
    });
    gui.show(egui_ctx.ctx_mut(), |ui| {
        ui.set_height(HEIGHT);
        ui.set_width(WIDTH);
        ui.style_mut()
            .override_font_id = Some(FontId::new(
                16.0,
                 egui::FontFamily::Monospace
        ));
        ui.horizontal(|ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(SPACING * 0.1);
                let back = ui.button("Back");
                if back.clicked() {
                    next_state.set(DisplayState::Main);
                }
                let (sec, min, hr) = rbr.telemetry.get_time();
                ui.label(format!("{min} : {sec}"));
                ui.add_space(SPACING * 0.1);
                let (response, painter) = ui.allocate_painter(DELTA_SIZE, Sense::hover());
                let c = response.rect.center();
                painter.rect_filled(
                    Rect::from_center_size(
                        c,
                        DELTA_SIZE
                    ), 
                    Rounding::same(0.0),
                    Color32::GRAY
                );
                let offset = -1.0;
                let delta_color = if offset > 0.0 {
                    Color32::GREEN
                } else {
                    Color32::RED
                };
                let mut pos = c;
                pos.y -= DELTA_SIZE.y / 2.0;
                let mut size = Pos2::new(pos.x + offset * 100.0, pos.y + DELTA_SIZE.y);
                if offset * 100.0 >= DELTA_SIZE.x / 2.0 {
                    size.x = pos.x + (DELTA_SIZE.x / 2.0);
                }
                if offset * 100.0 <= -DELTA_SIZE.x / 2.0 {
                    size.x = pos.x - (DELTA_SIZE.x / 2.0);
                }
                painter.rect_filled(
                    Rect::from_two_pos(
                        pos,
                        size
                    ),
                    Rounding::same(0.0),
                    delta_color, 
                );
                ui.style_mut()
                    .override_font_id = Some(FontId::new(50.0,egui::FontFamily::Monospace
                ));
                ui.colored_label(delta_color, format!("{offset}"));
                ui.style_mut()
                    .override_font_id = Some(FontId::new(16.0,egui::FontFamily::Monospace
                ));
                let best_time: &str = "2:55.332";
                ui.colored_label(Color32::GREEN, format!("Best time: {best_time}"));
            });
        });
    });
}




fn pedal_menu(
    mut windows: Query<&mut Window>,
    mut egui_ctx: EguiContexts,
    mut next_state: ResMut<NextState<DisplayState>>,
    rbr: Res<RBR>,
    pedals: Res<Pedals>,
    mut checkboxes: ResMut<PedalCheckboxes>
) {
    let mut window = windows.single_mut();
    window.resolution.set(WIDTH * 1.5, HEIGHT / 2.0);
    let gui = egui::Window::new("gui")
        .title_bar(false)
        .fixed_pos(ZERO)
        .default_height(HEIGHT)
        .default_width(WIDTH)
        .collapsible(false)
        .resizable(false)
        .frame(Frame {
            inner_margin: Margin::same(0.0),
            outer_margin: Margin::same(0.0),
            ..default()
        });
    
    gui.show(egui_ctx.ctx_mut(), |ui| {
        ui.set_height(HEIGHT);
        ui.set_width(WIDTH);
        ui.style_mut()
            .override_font_id = Some(FontId::new(
                16.0,
                 egui::FontFamily::Monospace
        ));
        ui.horizontal(|ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(SPACING * 0.1);
                ui.horizontal(|ui| {
                    ui.add_space(HORIZONTAL_CENTER * 4.0 + 10.0);
                    let back = ui.button("back");
                    if back.clicked() {
                        next_state.set(DisplayState::Main);
                    }
                });
                ui.horizontal(|ui| {
                    ui.add_space(25.0);
                    ui.vertical(|ui| {
                        ui.colored_label(Color32::GREEN, "Throttle");
                        ui.add(egui::Checkbox::without_text(&mut checkboxes.throttle));
                    });
                    ui.vertical(|ui| {
                        ui.colored_label(Color32::RED, "Brake");
                        ui.add(egui::Checkbox::without_text(&mut checkboxes.brake));
                    });
                    ui.vertical(|ui| {
                        ui.colored_label(Color32::BLUE, "Handbrake");
                        ui.add(egui::Checkbox::without_text(&mut checkboxes.handbrake));
                    });
                    ui.vertical(|ui| {
                        ui.colored_label(Color32::LIGHT_BLUE, "Clutch");
                        ui.add(egui::Checkbox::without_text(&mut checkboxes.clutch));
                    });
                    ui.vertical(|ui| {
                        ui.colored_label(Color32::YELLOW, "Gear");
                        ui.add(egui::Checkbox::without_text(&mut checkboxes.gear));
                    });
                });
            });
        });
        
        ui.vertical(|ui| {
            for i in 0..pedals.size {
                if checkboxes.throttle {
                    create_dot(
                        ui, 
                        ((i * DOT_SPACING) as f32),
                        (GRAPH_SIZE.y - (pedals.throttle[i as usize] + 5.0)),
                        Color32::GREEN
                    );
                }
                if checkboxes.brake {
                    create_dot(
                        ui, 
                        ((i * DOT_SPACING) as f32), 
                        (GRAPH_SIZE.y - (pedals.brake[i as usize] + 5.0)),
                        Color32::RED
                    );
                }
                if checkboxes.handbrake {
                    create_dot(
                        ui, 
                        ((i * DOT_SPACING) as f32), 
                        (GRAPH_SIZE.y - (pedals.handbrake[i as usize] + 5.0)),
                        Color32::BLUE
                    );
                }
                if checkboxes.clutch {
                    create_dot(
                        ui, 
                        ((i * DOT_SPACING) as f32), 
                        (GRAPH_SIZE.y - (pedals.clutch[i as usize] + 5.0)),
                        Color32::LIGHT_BLUE
                    );
                }
                if checkboxes.gear {
                    create_dot(
                        ui, 
                        ((i * DOT_SPACING) as f32), 
                        (GRAPH_SIZE.y - ((pedals.handbrake[i as usize] + 5.0) * 10.0)),
                        Color32::YELLOW
                    );
                }
            }
        });
    });
}

fn create_dot(
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

fn create_tire(
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

fn create_brake(
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

fn tire_menu(
    mut egui_ctx: EguiContexts,
    mut next_state: ResMut<NextState<DisplayState>>,
    rbr: Res<RBR>
) {
    let gui = egui::Window::new("gui")
        .title_bar(false)
        .fixed_pos(ZERO)
        .default_height(HEIGHT)
        .default_width(WIDTH)
        .collapsible(false)
        .resizable(false);
    gui.show(egui_ctx.ctx_mut(), |ui| {
        ui.style_mut()
            .override_font_id = Some(FontId::new(
                20.0,
                 egui::FontFamily::Monospace
        ));
        ui.vertical_centered(|ui| {
            let back = ui.button("Back");
            if back.clicked() {
                next_state.set(DisplayState::Main);
            }
        });
        ui.vertical(|ui| {
                ui.add_space(VERTICAL_CENTER);
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(TIRE_HORIZONTAL_SPACING);
                        let lf_brake = rbr.telemetry.car.suspension_lf.wheel.brake_disk.temperature;
                        let lf_tire = rbr.telemetry.car.suspension_lf.wheel.tire.temperature;
                        let rf_brake = rbr.telemetry.car.suspension_rf.wheel.brake_disk.temperature;
                        let rf_tire = rbr.telemetry.car.suspension_rf.wheel.tire.temperature;
                        ui.vertical(|ui| {
                            ui.add_space(BRAKE_VERTICAL_SPACING);
                            create_brake(ui, lf_brake);
                        });
                        ui.add_space(BRAKE_SPACING);
                        create_tire(ui, lf_tire);
                        ui.add_space(SPACING);
                        create_tire(ui, rf_tire);
                        ui.add_space(BRAKE_SPACING);
                        create_brake(ui, rf_brake);
                    });
                    ui.add_space(SPACING);
                    ui.horizontal(|ui| {
                        ui.add_space(TIRE_HORIZONTAL_SPACING);
                        let lb_brake = rbr.telemetry.car.suspension_lb.wheel.brake_disk.temperature;
                        let lb_tire = rbr.telemetry.car.suspension_lb.wheel.tire.temperature;
                        let rb_brake = rbr.telemetry.car.suspension_rb.wheel.brake_disk.temperature;
                        let rb_tire = rbr.telemetry.car.suspension_rb.wheel.tire.temperature;
                        ui.vertical(|ui| {
                            ui.add_space(BRAKE_VERTICAL_SPACING);
                            create_brake(ui, lb_brake);
                        });
                        ui.add_space(BRAKE_SPACING);
                        create_tire(ui, lb_tire);
                        ui.add_space(SPACING);
                        create_tire(ui, rb_tire);
                        ui.add_space(BRAKE_SPACING);
                        create_brake(ui, rb_brake);
                    });
                });
                ui.add_space(VERTICAL_CENTER * 5.0);

        });  
    });
}

fn get_color(temperature: f32) -> Color32 {
    if temperature > MAX_TEMP {
        return Color32::LIGHT_GREEN;
    }
    if temperature < MIN_TEMP {
        return Color32::DARK_BLUE;
    }
    let temp: u8 = (temperature - 273.15) as u8;
    let mut g: u8 = 255 - temp;
    let mut b: u8 = temp;
    Color32::from_rgb(0, g, b)
}


fn main_menu(
    mut windows: Query<&mut Window>,
    mut egui_ctx: EguiContexts,
    mut next_state: ResMut<NextState<DisplayState>>,
    mut connection_state: ResMut<NextState<ConnectionState>>,
    connection_state_current: Res<State<ConnectionState>>,
    mut port: ResMut<Port>,
    socket: Res<Socket>,
    rbr: Res<RBR>
) {
    let mut window = windows.single_mut();
    window.resolution.set(WIDTH, HEIGHT);
    let gui = egui::Window::new("gui")
        .title_bar(false)
        .fixed_pos(ZERO)
        .default_height(HEIGHT)
        .default_width(WIDTH)
        .collapsible(false)
        .resizable(false);
    gui.show(egui_ctx.ctx_mut(), |ui| {
        ui.style_mut()
            .override_font_id = Some(FontId::new(
                20.0,
                 egui::FontFamily::Monospace
        ));
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.style_mut().visuals.menu_rounding = Rounding::same(0.0);
            ui.style_mut().visuals.extreme_bg_color = Color32::BLACK;
            ui.set_height(HEIGHT);
            ui.add_space(SPACING);
            ui.label("RBR-GUI");
            ui.label("Developed by");
            ui.hyperlink_to("Maj Guček", "https://github.com/MajGucek/RBR-GUI");
            ui.add_space(SPACING);
            let pedals = ui.button("Pedal Telemetry");
            let tires = ui.button("Tyre Telemetry");
            let delta = ui.button("Delta Time");
            
            ui.add_space(SPACING);
            let p = &socket.address;
            match connection_state_current.get() {
                ConnectionState::Connected => {
                    ui.colored_label(Color32::GREEN, format!("{p}"));
                },
                ConnectionState::Disconnected => {
                    ui.label(
                        format!("Waiting connection!")
                    );
                }
            }
            
            if rbr.recv {
                ui.colored_label(Color32::GREEN, "Connected!");
            } else {
                ui.colored_label(Color32::RED, "Not connected!");
            };
            
            if rbr.recv {
                let (sec, min, hr) = rbr.telemetry.get_time();
                println!("{hr}: {min}: {sec}");
                ui.label(format!("Race time: {}", sec));
            }
            
            

            let response = ui.add(
                egui::TextEdit::singleline(&mut port.port)
                .hint_text("UDP port")
            );
            
            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                connection_state.set(ConnectionState::Disconnected);
            }
            

            if pedals.clicked() {
                next_state.set(DisplayState::Pedals);
            }
            if tires.clicked() {
                next_state.set(DisplayState::Tyres);
            }
            
            if delta.clicked() {
                next_state.set(DisplayState::Delta);
            }
            
        });
            
    });
}


fn connect_udp(
    mut socket: ResMut<Socket>,
    mut next_state: ResMut<NextState<ConnectionState>>,
    port: Res<Port>
) {
    let p = &port.port;
    socket.bind(p);
    match socket.socket {
        Ok(_) => {
            next_state.set(ConnectionState::Connected);
        },
        Err(_) => {
            next_state.set(ConnectionState::Disconnected);
        },
    }
}

fn telemetry_handler(
    mut rbr: ResMut<RBR>,
    socket: Res<Socket>,
    mut next_state: ResMut<NextState<ConnectionState>>,
    mut pedals: ResMut<Pedals>,
    port: Res<Port>
) {
    
    let mut buf = [0; 664];
    let mut socket = &socket.socket.as_ref();
    match socket.ok() {
        Some(udp_socket) => {
            udp_socket.set_nonblocking(true)
                .expect("Failed to enter non-blocking mode");
            match udp_socket.recv(&mut buf).ok() {
                Some(_) => {
                    println!("Received data!");
                    rbr.recv = true;
                    rbr.get_data(&buf);
                    pedals.add_data(&rbr.telemetry.control);
                },
                None => {
                    rbr.recv = false;
                    //println!("Failed recv()");
                }
            }
            
            
        },
        None => {
            next_state.set(ConnectionState::Disconnected);
        },
    }
}