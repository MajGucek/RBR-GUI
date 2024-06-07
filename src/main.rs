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


const ADDR: &str = "127.0.0.1:";
const WIDTH: f32 = 400.0;
const HEIGHT: f32 = 400.0;
const ZERO: Pos2 = Pos2::new(0.0, 0.0);
const HORIZONTAL_CENTER: f32 = 70.0;
const VERTICAL_CENTER: f32 = 50.0;
const SPACING: f32 = 50.0;
const TIRE_SIZE: Vec2 = Vec2::splat(100.0);
const GRAPH_SIZE: Vec2 = Vec2::new(WIDTH * 2.0, 200.0);
const DOT_SIZE: Vec2 = Vec2::splat(1.0);
const DOT_SPACING: u32 = 1;
const CHECKBOX_SPACING: f32 = 10.0;
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
            throttle: VecDeque::from([
                100.0,
99.5,
99.0,
98.5,
98.0,
97.5,
97.0,
96.5,
96.0,
95.5,
95.0,
94.5,
94.0,
93.5,
93.0,
92.5,
92.0,
91.5,
91.0,
90.5,
90.0,
89.5,
89.0,
88.5,
88.0,
87.5,
87.0,
86.5,
86.0,
85.5,
85.0,
84.5,
84.0,
83.5,
83.0,
82.5,
82.0,
81.5,
81.0,
80.5,
80.0,
79.5,
79.0,
78.5,
78.0,
77.5,
77.0,
76.5,
76.0,
75.5,
75.0,
74.5,
74.0,
73.5,
73.0,
72.5,
72.0,
71.5,
71.0,
70.5,
70.0,
69.5,
69.0,
68.5,
68.0,
67.5,
67.0,
66.5,
66.0,
65.5,
65.0,
64.5,
64.0,
63.5,
63.0,
62.5,
62.0,
61.5,
61.0,
60.5,
60.0,
59.5,
59.0,
58.5,
58.0,
57.5,
57.0,
56.5,
56.0,
55.5,
55.0,
54.5,
54.0,
53.5,
53.0,
52.5,
52.0,
51.5,
51.0,
50.5,
50.0,
49.5,
49.0,
48.5,
48.0,
47.5,
47.0,
46.5,
46.0,
45.5,
45.0,
44.5,
44.0,
43.5,
43.0,
42.5,
42.0,
41.5,
41.0,
40.5,
40.0,
39.5,
39.0,
38.5,
38.0,
37.5,
37.0,
36.5,
36.0,
35.5,
35.0,
34.5,
34.0,
33.5,
33.0,
32.5,
32.0,
31.5,
31.0,
30.5,
30.0,
29.5,
29.0,
28.5,
28.0,
27.5,
27.0,
26.5,
26.0,
25.5,
25.0,
24.5,
24.0,
23.5,
23.0,
22.5,
22.0,
21.5,
21.0,
20.5,
20.0,
19.5,
19.0,
18.5,
18.0,
17.5,
17.0,
16.5,
16.0,
15.5,
15.0,
14.5,
14.0,
13.5,
13.0,
12.5,
12.0,
11.5,
11.0,
10.5,
10.0,
9.5,
9.0,
8.5,
8.0,
7.5,
7.0,
6.5,
6.0,
5.5,
5.0,
4.5,
4.0,
3.5,
3.0,
2.5,
2.0,
1.5,
1.0,
0.5,
0.0,
0.0,
0.5,
1.5,
1.0,
2.0,
1.5,
2.5,
3.5,
3.0,
4.0,
3.5,
4.5,
5.0,
4.5,
5.5,
6.5,
6.0,
7.0,
6.5,
7.5,
8.5,
8.0,
9.0,
8.5,
9.5,
10.5,
10.0,
11.0,
10.5,
11.5,
12.0,
11.5,
12.5,
13.5,
13.0,
14.0,
13.5,
14.5,
15.0,
14.5,
15.5,
16.5,
16.0,
17.0,
16.5,
17.5,
18.5,
18.0,
19.0,
18.5,
19.5,
20.5,
20.0,
21.0,
20.5,
21.5,
22.0,
21.5,
22.5,
23.5,
23.0,
24.0,
23.5,
24.5,
25.0,
24.5,
25.5,
26.5,
26.0,
27.0,
26.5,
27.5,
28.5,
28.0,
29.0,
28.5,
29.5,
30.5,
30.0,
31.0,
30.5,
31.5,
32.0,
31.5,
32.5,
33.5,
33.0,
34.0,
33.5,
34.5,
35.0,
34.5,
35.5,
36.5,
36.0,
37.0,
36.5,
37.5,
38.5,
38.0,
39.0,
38.5,
39.5,
40.5,
40.0,
41.0,
40.5,
41.5,
42.0,
41.5,
42.5,
43.5,
43.0,
44.0,
43.5,
44.5,
45.0,
44.5,
45.5,
46.5,
46.0,
47.0,
46.5,
47.5,
48.5,
48.0,
49.0,
48.5,
49.5,
50.0,
49.5,
50.5,
51.5,
51.0,
52.0,
51.5,
52.5,
53.5,
53.0,
54.0,
53.5,
54.5,
55.0,
54.5,
55.5,
56.5,
56.0,
57.0,
56.5,
57.5,
58.5,
58.0,
59.0,
58.5,
59.5,
60.5,
60.0,
61.0,
60.5,
61.5,
62.0,
61.5,
62.5,
63.5,
63.0,
64.0,
63.5,
64.5,
65.0,
64.5,
65.5,
66.5,
66.0,
67.0,
66.5,
67.5,
68.5,
68.0,
69.0,
68.5,
69.5,
70.5,
70.0,
71.0,
70.5,
71.5,
72.0,
71.5,
72.5,
73.5,
73.0,
74.0,
73.5,
74.5,
75.0,
74.5,
75.5,
76.5,
76.0,
77.0,
76.5,
77.5,
78.5,
78.0,
79.0,
78.5,
79.5,
80.5,
80.0,
81.0,
80.5,
81.5,
82.0,
81.5,
82.5,
83.5,
83.0,
84.0,
83.5,
84.5,
85.0,
84.5,
85.5,
86.5,
86.0,
87.0,
86.5,
87.5,
88.5,
88.0,
89.0,
88.5,
89.5,
90.5,
90.0,
91.0,
90.5,
91.5,
92.0,
91.5,
92.5,
93.5,
93.0,
94.0,
93.5,
94.5,
95.0,
94.5,
95.5,
96.5,
96.0,
97.0,
96.5,
97.5,
98.5,
98.0,
99.0,
98.5,
99.5,
100.0,
100.0,
99.0,
99.5,
98.5,
98.0,
97.5,
96.5,
97.0,
96.0,
95.5,
94.5,
95.0,
94.0,
93.5,
92.5,
93.0,
92.0,
91.5,
90.5,
91.0,
90.0,
89.5,
88.5,
89.0,
88.0,
87.5,
86.5,
87.0,
86.0,
85.5,
84.5,
85.0,
84.0,
83.5,
82.5,
83.0,
82.0,
81.5,
80.5,
81.0,
80.0,
79.5,
78.5,
79.0,
78.0,
77.5,
76.5,
77.0,
76.0,
75.5,
74.5,
75.0,
74.0,
73.5,
72.5,
73.0,
72.0,
71.5,
70.5,
71.0,
70.0,
69.5,
68.5,
69.0,
68.0,
67.5,
66.5,
67.0,
66.0,
65.5,
64.5,
65.0,
64.0,
63.5,
62.5,
63.0,
62.0,
61.5,
60.5,
61.0,
60.0,
59.5,
58.5,
59.0,
58.0,
57.5,
56.5,
57.0,
56.0,
55.5,
54.5,
55.0,
54.0,
53.5,
52.5,
53.0,
52.0,
51.5,
50.5,
51.0,
50.0,
49.5,
48.5,
49.0,
48.0,
47.5,
46.5,
47.0,
46.0,
45.5,
44.5,
45.0,
44.0,
43.5,
42.5,
43.0,
42.0,
41.5,
40.5,
41.0,
40.0,
39.5,
38.5,
39.0,
38.0,
37.5,
36.5,
37.0,
36.0,
35.5,
34.5,
35.0,
34.0,
33.5,
32.5,
33.0,
32.0,
31.5,
30.5,
31.0,
76.5,
77.5,
78.5,
78.0,
79.0,
78.5,
79.5,
80.5,
80.0,
81.0,
80.5,
81.5,
82.0,
81.5,
82.5,
83.5,
83.0,
84.0,
83.5,
84.5,
85.0,
84.5,
85.5,
86.5,
86.0,
87.0,
86.5,
87.5,
88.5,
88.0,
89.0,
88.5,
89.5,
90.5,
90.0,
91.0,
90.5,
91.5,
92.0,
91.5,
92.5,
93.5,
93.0,
94.0,
93.5,
94.5,
95.0,
94.5,
95.5,
96.5,
96.0,
97.0,
96.5,
97.5,
98.5,
98.0,
99.0,
98.5,
99.5,
100.0,
100.0,
99.0,
99.5,
98.5,
98.0,
97.5,
96.5,
97.0,
96.0,
95.5,
94.5,
95.0,
94.0,
93.5,
92.5,
93.0,
92.0,
91.5,
90.5,
91.0,
90.0,
89.5,
88.5,
89.0,
88.0,
87.5,
86.5,
87.0,
86.0,
85.5,
84.5,
85.0,
84.0,
83.5,
82.5,
83.0,
82.0,
81.5,
80.5,
81.0,
80.0,
79.5,
78.5,
79.0,
78.0,
77.5,
76.5,
77.0,
76.0,
75.5,
74.5,
75.0,
74.0,
73.5,
72.5,
73.0,
72.0,
71.5,
70.5,
71.0,
70.0,
69.5,
68.5,
69.0,
68.0,
67.5,
66.5,
67.0,
66.0,
65.5,
64.5,
65.0,
64.0,
63.5,
62.5,
63.0,
62.0,
61.5,
60.5,
61.0,
60.0,
59.5,
58.5,
59.0,
58.0,
57.5,
56.5,
57.0,
56.0,
55.5,
54.5,
55.0,
54.0,
53.5,
52.5,
53.0,
52.0,
51.5,
50.5,
51.0,
50.0,
49.5,
48.5,
49.0,
48.0,
47.5,
46.5,
47.0,
46.0,
45.5,
44.5,
45.0,
44.0,
43.5,
42.5,
43.0,
42.0,
41.5,
40.5,
41.0,
40.0,
39.5,
38.5,
39.0,
38.0,
37.5,
36.5,
37.0,
36.0,
35.5,
34.5,
35.0,
34.0,
33.5,
32.5,
33.0,
32.0,
31.5,
30.5,
31.0,
            ]),
            brake: VecDeque::from([
                
76.5,
77.5,
78.5,
78.0,
79.0,
78.5,
79.5,
80.5,
80.0,
81.0,
80.5,
81.5,
82.0,
81.5,
82.5,
83.5,
83.0,
84.0,
83.5,
84.5,
85.0,
84.5,
85.5,
86.5,
86.0,
87.0,
86.5,
87.5,
88.5,
88.0,
89.0,
88.5,
89.5,
90.5,
90.0,
91.0,
90.5,
91.5,
92.0,
91.5,
92.5,
93.5,
93.0,
94.0,
93.5,
94.5,
95.0,
94.5,
95.5,
96.5,
96.0,
97.0,
96.5,
97.5,
98.5,
98.0,
99.0,
98.5,
99.5,
100.0,
100.0,
99.0,
99.5,
98.5,
98.0,
97.5,
96.5,
97.0,
96.0,
95.5,
94.5,
95.0,
94.0,
93.5,
92.5,
93.0,
92.0,
91.5,
90.5,
91.0,
90.0,
89.5,
88.5,
89.0,
88.0,
87.5,
86.5,
87.0,
86.0,
85.5,
84.5,
85.0,
84.0,
83.5,
82.5,
83.0,
82.0,
81.5,
80.5,
81.0,
80.0,
79.5,
78.5,
79.0,
78.0,
77.5,
76.5,
77.0,
76.0,
75.5,
74.5,
75.0,
74.0,
73.5,
72.5,
73.0,
72.0,
71.5,
70.5,
71.0,
70.0,
69.5,
68.5,
69.0,
68.0,
67.5,
66.5,
67.0,
66.0,
65.5,
64.5,
65.0,
64.0,
63.5,
62.5,
63.0,
62.0,
61.5,
60.5,
61.0,
60.0,
59.5,
58.5,
59.0,
58.0,
57.5,
56.5,
57.0,
56.0,
55.5,
54.5,
55.0,
54.0,
53.5,
52.5,
53.0,
52.0,
51.5,
50.5,
51.0,
50.0,
49.5,
48.5,
49.0,
48.0,
47.5,
46.5,
47.0,
46.0,
45.5,
44.5,
45.0,
44.0,
43.5,
42.5,
43.0,
42.0,
41.5,
40.5,
41.0,
40.0,
39.5,
38.5,
39.0,
38.0,
37.5,
36.5,
37.0,
36.0,
35.5,
34.5,
35.0,
34.0,
33.5,
32.5,
33.0,
32.0,
31.5,
30.5,
31.0,
76.5,
77.5,
78.5,
78.0,
79.0,
78.5,
79.5,
80.5,
80.0,
81.0,
80.5,
81.5,
82.0,
81.5,
82.5,
83.5,
83.0,
84.0,
83.5,
84.5,
85.0,
84.5,
85.5,
86.5,
86.0,
87.0,
86.5,
87.5,
88.5,
88.0,
89.0,
88.5,
89.5,
90.5,
90.0,
91.0,
90.5,
91.5,
92.0,
91.5,
92.5,
93.5,
93.0,
94.0,
93.5,
94.5,
95.0,
94.5,
95.5,
96.5,
96.0,
97.0,
96.5,
97.5,
98.5,
98.0,
99.0,
98.5,
99.5,
100.0,
100.0,
99.0,
99.5,
98.5,
98.0,
97.5,
96.5,
97.0,
96.0,
95.5,
94.5,
95.0,
94.0,
93.5,
92.5,
93.0,
92.0,
91.5,
90.5,
91.0,
90.0,
89.5,
88.5,
89.0,
88.0,
87.5,
86.5,
87.0,
86.0,
85.5,
84.5,
85.0,
84.0,
83.5,
82.5,
83.0,
82.0,
81.5,
80.5,
81.0,
80.0,
79.5,
78.5,
79.0,
78.0,
77.5,
76.5,
77.0,
76.0,
75.5,
74.5,
75.0,
74.0,
73.5,
72.5,
73.0,
72.0,
71.5,
70.5,
71.0,
70.0,
69.5,
68.5,
69.0,
68.0,
67.5,
66.5,
67.0,
66.0,
65.5,
64.5,
65.0,
64.0,
63.5,
62.5,
63.0,
62.0,
61.5,
60.5,
61.0,
60.0,
59.5,
58.5,
59.0,
58.0,
57.5,
56.5,
57.0,
56.0,
55.5,
54.5,
55.0,
54.0,
53.5,
52.5,
53.0,
52.0,
51.5,
50.5,
51.0,
50.0,
49.5,
48.5,
49.0,
48.0,
47.5,
46.5,
47.0,
46.0,
45.5,
44.5,
45.0,
44.0,
43.5,
42.5,
43.0,
42.0,
41.5,
40.5,
41.0,
40.0,
39.5,
38.5,
39.0,
38.0,
37.5,
36.5,
37.0,
36.0,
35.5,
34.5,
35.0,
34.0,
33.5,
32.5,
33.0,
32.0,
31.5,
30.5,
31.0,
100.0,
99.5,
99.0,
98.5,
98.0,
97.5,
97.0,
96.5,
96.0,
95.5,
95.0,
94.5,
94.0,
93.5,
93.0,
92.5,
92.0,
91.5,
91.0,
90.5,
90.0,
89.5,
89.0,
88.5,
88.0,
87.5,
87.0,
86.5,
86.0,
85.5,
85.0,
84.5,
84.0,
83.5,
83.0,
82.5,
82.0,
81.5,
81.0,
80.5,
80.0,
79.5,
79.0,
78.5,
78.0,
77.5,
77.0,
76.5,
76.0,
75.5,
75.0,
74.5,
74.0,
73.5,
73.0,
72.5,
72.0,
71.5,
71.0,
70.5,
70.0,
69.5,
69.0,
68.5,
68.0,
67.5,
67.0,
66.5,
66.0,
65.5,
65.0,
64.5,
64.0,
63.5,
63.0,
62.5,
62.0,
61.5,
61.0,
60.5,
60.0,
59.5,
59.0,
58.5,
58.0,
57.5,
57.0,
56.5,
56.0,
55.5,
55.0,
54.5,
54.0,
53.5,
53.0,
52.5,
52.0,
51.5,
51.0,
50.5,
50.0,
49.5,
49.0,
48.5,
48.0,
47.5,
47.0,
46.5,
46.0,
45.5,
45.0,
44.5,
44.0,
43.5,
43.0,
42.5,
42.0,
41.5,
41.0,
40.5,
40.0,
39.5,
39.0,
38.5,
38.0,
37.5,
37.0,
36.5,
36.0,
35.5,
35.0,
34.5,
34.0,
33.5,
33.0,
32.5,
32.0,
31.5,
31.0,
30.5,
30.0,
29.5,
29.0,
28.5,
28.0,
27.5,
27.0,
26.5,
26.0,
25.5,
25.0,
24.5,
24.0,
23.5,
23.0,
22.5,
22.0,
21.5,
21.0,
20.5,
20.0,
19.5,
19.0,
18.5,
18.0,
17.5,
17.0,
16.5,
16.0,
15.5,
15.0,
14.5,
14.0,
13.5,
13.0,
12.5,
12.0,
11.5,
11.0,
10.5,
10.0,
9.5,
9.0,
8.5,
8.0,
7.5,
7.0,
6.5,
6.0,
5.5,
5.0,
4.5,
4.0,
3.5,
3.0,
2.5,
2.0,
1.5,
1.0,
0.5,
0.0,
0.0,
0.5,
1.5,
1.0,
2.0,
1.5,
2.5,
3.5,
3.0,
4.0,
3.5,
4.5,
5.0,
4.5,
5.5,
6.5,
6.0,
7.0,
6.5,
7.5,
8.5,
8.0,
9.0,
8.5,
9.5,
10.5,
10.0,
11.0,
10.5,
11.5,
12.0,
11.5,
12.5,
13.5,
13.0,
14.0,
13.5,
14.5,
15.0,
14.5,
15.5,
16.5,
16.0,
17.0,
16.5,
17.5,
18.5,
18.0,
19.0,
18.5,
19.5,
20.5,
20.0,
21.0,
20.5,
21.5,
22.0,
21.5,
22.5,
23.5,
23.0,
24.0,
23.5,
24.5,
25.0,
24.5,
25.5,
26.5,
26.0,
27.0,
26.5,
27.5,
28.5,
28.0,
29.0,
28.5,
29.5,
30.5,
30.0,
31.0,
30.5,
31.5,
32.0,
31.5,
32.5,
33.5,
33.0,
34.0,
33.5,
34.5,
35.0,
34.5,
35.5,
36.5,
36.0,
37.0,
36.5,
37.5,
38.5,
38.0,
39.0,
38.5,
39.5,
40.5,
40.0,
41.0,
40.5,
41.5,
42.0,
41.5,
42.5,
43.5,
43.0,
44.0,
43.5,
44.5,
45.0,
44.5,
45.5,
46.5,
46.0,
47.0,
46.5,
47.5,
48.5,
48.0,
49.0,
48.5,
49.5,
50.0,
49.5,
50.5,
51.5,
51.0,
52.0,
51.5,
52.5,
53.5,
53.0,
54.0,
53.5,
54.5,
55.0,
54.5,
55.5,
56.5,
56.0,
57.0,
56.5,
57.5,
58.5,
58.0,
59.0,
58.5,
59.5,
60.5,
60.0,
61.0,
60.5,
61.5,
62.0,
61.5,
62.5,
63.5,
63.0,
64.0,
63.5,
64.5,
65.0,
64.5,
65.5,
66.5,
66.0,
67.0,
66.5,
67.5,
68.5,
68.0,
69.0,
68.5,
69.5,
70.5,
70.0,
71.0,
70.5,
71.5,
72.0,
71.5,
72.5,
73.5,
73.0,
74.0,
73.5,
74.5,
75.0,
74.5,
75.5,
76.5,
76.0,
        ]),
            clutch: VecDeque::new(),
            gear: VecDeque::new(),
            handbrake: VecDeque::new(),
            size: 800,
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
                16.0,
                 egui::FontFamily::Monospace
        ));
        ui.horizontal(|ui| {
            //
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(HORIZONTAL_CENTER * 4.0);
                    ui.label("Pedals");
                });
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
                            /*
                            let lf = rbr
                                .telemetry
                                .car
                                .suspension_lf
                                .wheel
                                .tire
                                .temperature;
                            */
                            create_tyre(ui, 370.0);
                            ui.add_space(SPACING);
                        });
                        ui.add_space(SPACING);
                        ui.vertical(|ui| {
                            /*
                            let rf = rbr
                                .telemetry
                                .car
                                .suspension_rf
                                .wheel
                                .tire
                                .temperature;
                            */
                            create_tyre(ui, 300.0);
                        });
                    })
                    
                });
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(HORIZONTAL_CENTER);
                        ui.vertical(|ui| {
                            /*
                            let lb = rbr
                                .telemetry
                                .car
                                .suspension_lb
                                .wheel
                                .tire
                                .temperature;
                            */
                            create_tyre(ui, 340.0);
                            ui.add_space(SPACING);
                        });
                        ui.add_space(SPACING);
                        ui.vertical(|ui| {
                            /*
                            let rb = rbr
                                .telemetry
                                .car
                                .suspension_rb
                                .wheel
                                .tire
                                .temperature;
                            */
                            create_tyre(ui, 400.0);
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
                        format!("{p}")
                    );
                },
                ConnectionState::Disconnected => {
                    ui.label(
                        format!("Waiting connection!")
                    );
                }
            }
            let color = if rbr.recv {
                Color32::GREEN
            } else {
                Color32::RED
            };
            ui.colored_label(color, "Connection State");
            let time = if rbr.recv {
                rbr.telemetry.stage.race_time
            } else {
                -1.0
            };
            ui.label(format!("Race time: {time}"));

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
    mut rbr: ResMut<RBR>,
    socket: Res<Socket>,
    mut next_state: ResMut<NextState<ConnectionState>>,
    mut pedals: ResMut<Pedals>
) {
    let mut buf = [0; 664];
    match socket.socket.as_ref().ok() {
        Some(udp_socket) => {
            udp_socket.set_nonblocking(true)
                .expect("Failed to enter non-blocking mode");
            match udp_socket.recv(&mut buf).ok() {
                Some(_) => {
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