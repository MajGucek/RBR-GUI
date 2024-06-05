use std::net::UdpSocket;
use std::io::Error;
use bincode::deserialize;
mod telemetry;
use telemetry::Telemetry;

// UI
use bevy::{prelude::*, ui::ContentSize};
use bevy_egui::{EguiContexts, EguiPlugin};
//use bevy::time::common_conditions::on_timer;
//use bevy::utils::Duration;

const PORT: &str = "127.0.0.1:6779";


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

impl Default for Socket {
    fn default() -> Self {
        Socket {
            socket: UdpSocket::bind(PORT),
            paired: false,
        }
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
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
        .add_systems(Update, ui_handler)
        .run();
}


fn ui_handler(
    mut contexts: EguiContexts,

    rbr: Res<RBR>
) {
    
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {

    });
    
}


fn connect_udp(
    mut socket: ResMut<Socket>
) {
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




