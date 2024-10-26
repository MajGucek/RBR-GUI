#[allow(dead_code)]

use bevy::prelude::*;
use std::net::UdpSocket;
use std::io::Error;
use bincode::deserialize;
use std::collections::VecDeque;

// telemetry.rs
use crate::Telemetry;
use crate::Control;
// constants.rs
use crate::constants::*;

#[derive(Resource)]
pub struct RBR {
    pub telemetry: Telemetry,
    pub recv: bool,
}
impl RBR {
    pub fn get_data(&mut self, data: &[u8]) {
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
pub struct Socket {
    pub socket: Result<UdpSocket, Error>,
    pub address: String,
}
impl Socket {
    pub fn bind(&mut self, port: &str) {
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
pub struct Port {
    pub port: String,
}
impl Default for Port {
    fn default() -> Self {
        Port {
            port: String::new(),
        }
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectionState {
    Disconnected,
    Connected,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DisplayState {
    Main,
    Tires,
    Pedals,
    Suspension
}

#[derive(Resource)]
pub struct Pedals {
    pub throttle: VecDeque<f32>,
    pub brake: VecDeque<f32>,
    pub handbrake: VecDeque<f32>,
    pub clutch: VecDeque<f32>,
    pub gear: VecDeque<i32>,
    pub size: u32,
}
impl Pedals {
    pub fn add_data(&mut self, data: &Control) {
        if self.size > ((GRAPH_SIZE.x) as u32) {
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
pub struct PedalCheckboxes {
    pub throttle: bool,
    pub brake: bool, 
    pub handbrake: bool,
    pub clutch: bool,
    pub gear: bool,
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

