use std::net::UdpSocket;
use bincode::deserialize;
mod telemetry;
use telemetry::Telemetry;

// UI
use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin};
use bevy::time::common_conditions::on_timer;
use bevy::utils::Duration;


#[derive(Resource)]
struct Connection {
    is_connected: bool,
}

#[derive(Resource)]
struct Data {
    buf: [u8; 664],
    telemetry: Telemetry,
}

#[derive(Event)]
struct UdpConnectedEvent(UdpSocket);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_event::<UdpConnectedEvent>()
        .add_systems(
            Update,
            (
                udp_connected.run_if(on_timer(Duration::from_secs(1))),
                handle_telemetry,
            )
        )
        .run();
}

fn udp_connected(
    mut connection: ResMut<Connection>,
    mut ev_udp_connected: EventWriter<UdpConnectedEvent>,
) {
    if !udp.paired {
        let socket = UdpSocket::bind("127.0.0.1:6779");
        match socket {
            Ok(_) => {
                ev_udp_connected.send(UdpConnectedEvent(socket.unwrap()));
            },
            Err(_) => {
                println!("Failed to Bind to Port 6779!");
            },
        }
    }
}

fn handle_telemetry(
    socket: Res<Connection>,
    mut data: ResMut<Data>
) {
    if socket.paired {
        match socket.socket.recv(&mut data.buf) {
            Ok(_) => { proccess_telemetry(&mut data)},
            Err(e) =>  { println!("recv function failed: {e:?}") },
        }
    }
}


fn proccess_telemetry(
    data: &mut Data
) {
    data.telemetry = deserialize(&data.buf).unwrap();
}
