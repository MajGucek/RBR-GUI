// Hide Terminal
#![windows_subsystem = "windows"]

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


const ADDR: &str = "127.0.0.1:";
const WIDTH: f32 = 400.0;
const HEIGHT: f32 = 400.0;
const ZERO: Pos2 = Pos2::new(0.0, 0.0);
const HORIZONTAL_CENTER: f32 = 70.0;
const VERTICAL_CENTER: f32 = 50.0;
const SPACING: f32 = 50.0;
const TIRE_SIZE: Vec2 = Vec2::splat(100.0);
const GRAPH_SIZE: Vec2 = Vec2::new(WIDTH * 1.5, 200.0);
const DOT_SIZE: Vec2 = Vec2::splat(5.0);
const MAX_TEMP: f32 = 150.0 + 273.15;
const MIN_TEMP: f32 = 40.0 + 273.15;

#[derive(Resource)]
struct RBR {
    telemetry: Telemetry,
}
impl RBR {
    fn get_data(&mut self, data: &[u8]) {
        self.telemetry = deserialize(&data).unwrap();
    }
}
impl Default for RBR {
    fn default() -> Self {
        RBR {
            telemetry: Telemetry::default(),
        }
    }
}


#[derive(Resource)]
struct Data {
    buf: [u8; 664],
}

impl Default for Data {
    fn default() -> Self {
        Data {
            buf: [0; 664],
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
        self.address = format!("{ADDR}{port}");
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
            port: String::with_capacity(15),
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
        if self.size > 1000 {
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
        .init_resource::<Data>()
        .init_resource::<RBR>()
        .init_resource::<Port>()
        .init_resource::<Pedals>()
        .add_systems(
            Update,
            (
                telemetry_handler
                    .run_if(in_state(ConnectionState::Connected))
                    .run_if(not(in_state(DisplayState::Main))),
                connect_udp
                    .run_if(in_state(ConnectionState::Disconnected))
                    .run_if(on_timer(Duration::from_secs(2)))
                    )
        )
        
        .add_systems(Update, 
            (
                main_menu.run_if(in_state(DisplayState::Main)),
                pedal_menu.run_if(in_state(DisplayState::Pedals)),
                tyre_menu.run_if(in_state(DisplayState::Tyres))
        )
    )
    .run();
}


fn pedal_menu(
    mut windows: Query<&mut Window>,
    mut egui_ctx: EguiContexts,
    mut next_state: ResMut<NextState<DisplayState>>,
    rbr: Res<RBR>,
    pedals: Res<Pedals>
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
        .min_height(HEIGHT)
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
                20.0,
                 egui::FontFamily::Monospace
        ));
        ui.horizontal(|ui| {
            ui.add_space(HORIZONTAL_CENTER * 3.0);
            ui.vertical_centered(|ui| {
                ui.label("Pedals");
                let back = ui.button("back");
                if back.clicked() {
                    next_state.set(DisplayState::Main);
                }
            });
        });
        
        ui.vertical(|ui| {
            let (response, painter) = ui.allocate_painter(GRAPH_SIZE, Sense::hover());
            painter.rect_filled(
                Rect::from_two_pos(
                    Pos2::new(0.0, 30.0),
                    Pos2::new(GRAPH_SIZE.x, (30.0 + GRAPH_SIZE.y)) 
                    ),
                    Rounding::same(0.0),
                    Color32::from_rgb(112,128,144)
            );
            for i in 0..pedals.size {
                create_dot(
                        ui, 
                    ((i * 10) as f32),
                    pedals.brake[i as usize],
                    Color32::RED
                );
                create_dot(
                    ui,
                    ((i * 10) as f32),
                    pedals.throttle[i as usize],
                    Color32::GREEN
                );
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
    let (response, painter) = ui.allocate_painter(DOT_SIZE, Sense::hover());
    let c = response.rect.center();
    painter.circle_filled(
        Pos2::new(x, y), 
        DOT_SIZE.x,
        color 
    );
}

fn create_tyre(
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

fn tyre_menu(
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
        .resizable(false)
        .min_height(HEIGHT);
    gui.show(egui_ctx.ctx_mut(), |ui| {
        ui.style_mut()
            .override_font_id = Some(FontId::new(
                20.0,
                 egui::FontFamily::Monospace
        ));
        ui.vertical_centered(|ui| {
            ui.label("Tyres");
            let back = ui.button("Back");
            if back.clicked() {
                next_state.set(DisplayState::Main);
            }
        });
        ui.vertical_centered(|ui| {
            ui.with_layout(
                egui::Layout::top_down_justified(egui::Align::Center),
                 |ui| {
                ui.add_space(VERTICAL_CENTER);
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(HORIZONTAL_CENTER);
                        ui.vertical(|ui| {
                            let lf = rbr
                                .telemetry
                                .car
                                .suspension_lf
                                .wheel
                                .tire
                                .temperature;
                            create_tyre(ui, lf);
                            ui.add_space(SPACING);
                        });
                        ui.add_space(SPACING);
                        ui.vertical(|ui| {
                            let rf = rbr
                                .telemetry
                                .car
                                .suspension_rf
                                .wheel
                                .tire
                                .temperature;
                            create_tyre(ui, rf);
                        });
                    })
                    
                });
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(HORIZONTAL_CENTER);
                        ui.vertical(|ui| {
                            let lb = rbr
                                .telemetry
                                .car
                                .suspension_lb
                                .wheel
                                .tire
                                .temperature;
                            create_tyre(ui, lb);
                            ui.add_space(SPACING);
                        });
                        ui.add_space(SPACING);
                        ui.vertical(|ui| {
                            let rb = rbr
                                .telemetry
                                .car
                                .suspension_rb
                                .wheel
                                .tire
                                .temperature;
                            create_tyre(ui, rb);
                        });
                    });
                    
                });
            });
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
    socket: Res<Socket>
) {
    let mut window = windows.single_mut();
    window.resolution.set(WIDTH, HEIGHT);
    let gui = egui::Window::new("gui")
        .title_bar(false)
        .fixed_pos(ZERO)
        .default_height(HEIGHT)
        .default_width(WIDTH)
        .collapsible(false)
        .resizable(false)
        .min_height(HEIGHT);
    
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
            let tyres = ui.button("Tyre Telemetry");
            ui.add_space(SPACING);
            let p = &socket.address;
            match connection_state_current.get() {
                ConnectionState::Connected => {
                    ui.label(
                        format!("Connected to: {p}!")
                    );
                },
                ConnectionState::Disconnected => {
                    ui.label(
                        format!("Waiting connection!")
                    );
                }
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
            if tyres.clicked() {
                next_state.set(DisplayState::Tyres);
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
    mut data: ResMut<Data>,
    mut rbr: ResMut<RBR>,
    socket: Res<Socket>,
    mut next_state: ResMut<NextState<ConnectionState>>,
    mut pedals: ResMut<Pedals>
) {
    match socket.socket.as_ref().expect("LMAO!").recv(&mut data.buf) {
        Ok(_) => { 
            rbr.get_data(&data.buf);
            pedals.add_data(&rbr.telemetry.control);
            next_state.set(ConnectionState::Connected);
        },
        Err(_) =>  { 
            println!("Couldn't read from UDP port! Retrying...");
            next_state.set(ConnectionState::Disconnected);
        },
    }
}