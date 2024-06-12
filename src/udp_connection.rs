use bevy::prelude::*;

// resources.rs
use crate::resources::*;


pub fn connect_udp(
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

pub fn telemetry_handler(
    mut rbr: ResMut<RBR>,
    socket: Res<Socket>,
    mut next_state: ResMut<NextState<ConnectionState>>,
    mut pedals: ResMut<Pedals>
) {
    
    let mut buf = [0; 664];
    let socket = &socket.socket.as_ref();
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