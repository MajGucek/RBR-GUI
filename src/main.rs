use std::net::UdpSocket;
use std::io::Error;
use bincode::deserialize;
// telemetry.rs
mod telemetry;
use telemetry::{Suspension, Telemetry};

extern crate colortemp;

// UI
use bevy::{prelude::*};
use bevy_egui::{EguiContexts, EguiPlugin};
use egui::{Color32, Frame, Mesh, Pos2, Rect, Rounding, Sense, Stroke, TextureId, Ui, Vec2, Widget, WidgetRect};
//use bevy::time::common_conditions::on_timer;
//use bevy::utils::Duration;

const PORT: &str = "127.0.0.1:6779";
const WIDTH: f32 = 400.0;
const HEIGHT: f32 = 400.0;
const ZERO: Pos2 = Pos2::new(0.0, 0.0);


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
    paired: bool,
}
impl Socket {
    fn bind(&mut self) {
        self.socket = UdpSocket::bind(PORT);
    }
}

impl Default for Socket {
    fn default() -> Self {
        Socket {
            socket: UdpSocket::bind(PORT),
            paired: false,
        }
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum DisplayState {
    Main,
    Tyres,
    Pedals,
}


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .insert_state(DisplayState::Main)
        .init_resource::<Socket>()
        .init_resource::<Data>()
        .init_resource::<RBR>()
        /*
        .add_systems(Startup, connect_udp)
        .add_systems(
            Update,
            (
                telemetry_handler,
                ui_handler
                    )
        )
        */
        .add_systems(Update, 
            (
                main_menu.run_if(in_state(DisplayState::Main)),
                pedal_menu.run_if(in_state(DisplayState::Pedals)),
                tyre_menu.run_if(in_state(DisplayState::Tyres))
        ))
        .add_systems(Startup, init)
        .run();
}

const HORIZONTAL_CENTER: f32 = 70.0;
const VERTICAL_CENTER: f32 = 50.0;
const SPACING: f32 = 50.0;

const SIZE: Vec2 = Vec2::splat(100.0);

fn tyre_menu(
    mut windows: Query<&mut Window>,
    mut egui_ctx: EguiContexts,
    mut next_state: ResMut<NextState<DisplayState>>,
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
        .resizable(false)
        .min_height(HEIGHT)
        .frame(Frame {
            ..default()
        });
    gui.show(egui_ctx.ctx_mut(), |ui| {
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
                            //ui.label("LF");
                            create_tyre(ui, rbr.telemetry.);
                            ui.add_space(SPACING);
                        });
                        ui.add_space(SPACING);
                        ui.vertical(|ui| {
                            //ui.label("RF");
                            create_tyre(ui);
                        });
                    })
                    
                });
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(HORIZONTAL_CENTER);
                        ui.vertical(|ui| {
                            //ui.label("LR");
                            create_tyre(ui);
                            ui.add_space(SPACING);
                        });
                        ui.add_space(SPACING);
                        ui.vertical(|ui| {
                            //ui.label("RR");
                            create_tyre(ui);
                        });
                    });
                    
                });
            });
        });  
    });
}

fn create_tyre(
    ui: &mut Ui,
    temperature: f32,
) {
    let (response, painter) = ui.allocate_painter(SIZE, Sense::hover());
    let c = response.rect.center();
    painter.rect_filled(
        Rect::from_center_size(
            c,
            SIZE
        ), 
        Rounding::same(0.0),
        Color32::GREEN 
    );
}

fn get_color(temperature: f32) -> Color32 {

}


fn pedal_menu(
    mut windows: Query<&mut Window>,
    mut egui_ctx: EguiContexts,
    mut next_state: ResMut<NextState<DisplayState>>,
) {
    let mut window = windows.single_mut();
    window.resolution.set(WIDTH, HEIGHT / 2.0);
}


fn main_menu(
    mut windows: Query<&mut Window>,
    mut egui_ctx: EguiContexts,
    mut next_state: ResMut<NextState<DisplayState>>,
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
        .min_height(HEIGHT)
        .frame(Frame {
            ..default()
        });
    
    gui.show(egui_ctx.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label("Main Menu");
            let pedals = ui.button("Pedal Telemetry");
            let tyres = ui.button("Tyre Telemetry");
            if pedals.clicked() {
                next_state.set(DisplayState::Pedals);
            }
            if tyres.clicked() {
                next_state.set(DisplayState::Tyres);
            }
        });
            
    });
}


fn init(
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();
    window.resolution.set(WIDTH, HEIGHT);
    window.position = WindowPosition::new(IVec2 {x: 0, y: 0});
    window.title = "RBR-GUI".to_string();
    window.resize_constraints.min_width = WIDTH;
    window.resize_constraints.min_height = HEIGHT;
}

fn connect_udp(
    mut socket: ResMut<Socket>
) {
    socket.bind();
    match socket.socket {
        Ok(_) => {
            socket.paired = true;
        },
        Err(_) => {
            socket.paired = false;
        },
    }
}

fn telemetry_handler(
    mut data: ResMut<Data>,
    mut rbr: ResMut<RBR>,
    socket: Res<Socket>
) {
    if socket.paired {
        match socket.socket.as_ref().expect("Error").recv(&mut data.buf) {
            Ok(_) => { 
                rbr.get_data(&data.buf);
            },
            Err(e) =>  { println!("recv function failed: {e:?}") },
        }
    } else {
        println!("Didn't connect to Port 6779!");
    }
}




