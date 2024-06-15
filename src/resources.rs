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
    Delta,
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

#[derive(Clone)]
pub struct TimePos {
    pub time: f32,
    pub pos: u32,
}

#[derive(Resource)]
pub struct BestTime {
    pub splits: Vec<TimePos>,
    pub final_time: f32, // seconds
    pub stage_index: i32,
}
impl BestTime {
    pub fn exists(&self) -> bool {
        self.final_time > 0.0
    }
    pub fn is_faster(&self, time: f32) -> bool {
        if time < self.final_time {
            true
        } else {
            false
        }
    }
    pub fn add_new_best_time(&mut self, time: f32, splits: &Vec<TimePos>, stage_index: i32) {
        self.final_time = time;
        self.splits = splits.clone();
        self.stage_index = stage_index;
    }
}
impl Default for BestTime {
    fn default() -> Self {
        BestTime {
            final_time: 0.0,
            splits: Vec::new(),
            stage_index: 0,
        }
    }
}


#[derive(Resource, Clone)]
pub struct CurrentTime {
    pub splits: Vec<TimePos>,
    pub stage_index: i32,
}
impl CurrentTime {
    pub fn reset(&mut self) {
        self.splits.clear();
        self.stage_index = 0;
    }
    pub fn add_split(&mut self, time: f32, pos: u32) {
        if pos == self.get_prev_pos() && self.splits.len() > 0 {
            self.splits.pop();
        }
        self.splits.push(TimePos {time, pos});
    }

    fn get_prev_pos(&self) -> u32 {
        if self.splits.len() > 0 {
            self.splits[self.splits.len() - 1].pos
        } else {
            0
        }
    }

}

impl Default for CurrentTime {
    fn default() -> Self {
        CurrentTime {
            splits: Vec::new(),
            stage_index: 0,
        }
    }
}



#[derive(Resource)]
pub struct DeltaTime {
    pub delta: f32,
}
impl Default for DeltaTime {
    fn default() -> Self {
        DeltaTime {
            delta: 0.0
        }
    }
}

