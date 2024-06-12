// Hide Terminal
#![windows_subsystem = "windows"]

// telemetry.rs
mod telemetry;
use telemetry::{Control, Telemetry};

// constants.rs
mod constants;
use constants::*;

// resources.rs
mod resources;
use resources::*;

// UI
use bevy::{
    prelude::*,
    window::WindowLevel,
    utils::Duration,
    time::common_conditions::on_timer
};
use bevy_egui::{
    EguiContexts, 
    EguiPlugin
};
use egui::{
    Color32, 
    FontId, 
    Frame, 
    Margin, 
    Pos2, 
    Rect, 
    Rounding, 
    Sense,
    Ui,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                position: WindowPosition::At(IVec2 { x: 10, y: 10 }),
                window_level: WindowLevel::AlwaysOnTop,
                ..default()
            }),
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
                tire_menu.run_if(in_state(DisplayState::Tires)),
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
        .frame(Frame {
            inner_margin: Margin::same(0.0),
            outer_margin: Margin::same(0.0),
            fill: MENU_BG,
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
            ui.vertical_centered(|ui| {
                ui.add_space(SPACING * 0.1);
                let back = ui.button("Back");
                if back.clicked() {
                    next_state.set(DisplayState::Main);
                }
                let time = rbr.telemetry.get_time();
                ui.label(format!("{} : {}", time.minutes, time.seconds));
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
                let offset = 0.0;
                let delta_color = if offset == 0.0 {
                    Color32::GRAY
                } else if offset > 0.0 {
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
    pedals: Res<Pedals>,
    mut checkboxes: ResMut<PedalCheckboxes>
) {
    let mut window = windows.single_mut();
    window.resolution.set(PEDAL_WIDTH, PEDAL_HEIGHT);
    let gui = egui::Window::new("gui")
        .title_bar(false)
        .fixed_pos(ZERO)
        .default_height(PEDAL_HEIGHT)
        .default_width(PEDAL_WIDTH)
        .collapsible(false)
        .frame(Frame {
            fill: MENU_BG,
            inner_margin: Margin::same(0.0),
            outer_margin: Margin::same(0.0),
            ..default()
        });
    
    gui.show(egui_ctx.ctx_mut(), |ui| {
        ui.set_height(PEDAL_HEIGHT);
        ui.set_width(PEDAL_WIDTH);
        ui.style_mut()
            .override_font_id = Some(FontId::new(
                16.0,
                 egui::FontFamily::Monospace
        ));
            ui.vertical_centered(|ui| {
                ui.add_space(SPACING * 0.1);
                let back = ui.button("back");
                if back.clicked() {
                    next_state.set(DisplayState::Main);
                }
            });
            ui.horizontal(|ui| {
                ui.add_space(HORIZONTAL_CENTER);
                ui.colored_label(Color32::GREEN, "Throttle");
                ui.add_space(CHECKBOX_SPACING);
                ui.colored_label(Color32::RED, "Brake");
                ui.add_space(CHECKBOX_SPACING);
                ui.colored_label(Color32::BLUE, "Handbrake");
                ui.add_space(CHECKBOX_SPACING);
                ui.colored_label(Color32::LIGHT_BLUE, "Clutch");
                ui.add_space(CHECKBOX_SPACING);
                ui.colored_label(Color32::YELLOW, "Gear");
            });
            ui.horizontal(|ui| {
                ui.add_space(HORIZONTAL_CENTER + 30.0);
                ui.add(egui::Checkbox::without_text(&mut checkboxes.throttle));
                ui.add_space(WORD_SPACING - 20.0);
                ui.add(egui::Checkbox::without_text(&mut checkboxes.brake));
                ui.add_space(WORD_SPACING - 12.0);
                ui.add(egui::Checkbox::without_text(&mut checkboxes.handbrake));
                ui.add_space(WORD_SPACING - 3.0);
                ui.add(egui::Checkbox::without_text(&mut checkboxes.clutch));
                ui.add_space(WORD_SPACING - 35.0);
                ui.add(egui::Checkbox::without_text(&mut checkboxes.gear));
            });
                
            
        
        ui.vertical(|ui| {
            create_line(ui, GRAPH_SIZE.y - DOT_SIZE.y);
            create_line(ui, GRAPH_SIZE.y - 33.4);
            create_line(ui, GRAPH_SIZE.y - 66.6);
            create_line(ui, GRAPH_SIZE.y - 100.0);
            for i in 0..pedals.size {
                if checkboxes.throttle {
                    create_dot(
                        ui, 
                        (i as f32) * DOT_SPACING,
                        GRAPH_SIZE.y - (pedals.throttle[i as usize]),
                        Color32::GREEN
                    );
                }
                if checkboxes.brake {
                    create_dot(
                        ui, 
                        (i as f32) * DOT_SPACING, 
                        GRAPH_SIZE.y - (pedals.brake[i as usize]),
                        Color32::RED
                    );
                }
                if checkboxes.handbrake {
                    create_dot(
                        ui, 
                        (i as f32) * DOT_SPACING, 
                        GRAPH_SIZE.y - (pedals.handbrake[i as usize]),
                        Color32::BLUE
                    );
                }
                if checkboxes.clutch {
                    create_dot(
                        ui, 
                        (i as f32) * DOT_SPACING, 
                        GRAPH_SIZE.y - (pedals.clutch[i as usize]),
                        Color32::LIGHT_BLUE
                    );
                }
                if checkboxes.gear {
                    create_dot(
                        ui, 
                        (i as f32) * DOT_SPACING, 
                        GRAPH_SIZE.y - (((pedals.gear[i as usize]) as f32) * 15.0),
                        Color32::YELLOW
                    );
                }
            }
        });
    });
}

fn create_line(
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
        .frame(Frame {
            fill: MENU_BG,
            inner_margin: Margin::same(0.0),
            outer_margin: Margin::same(0.0),
            ..default()
        });
    gui.show(egui_ctx.ctx_mut(), |ui| {
        ui.style_mut()
            .override_font_id = Some(FontId::new(
                20.0,
                 egui::FontFamily::Monospace
        ));
        ui.vertical_centered(|ui| {
            ui.add_space(SPACING * 0.1);
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
    let g: u8 = 255 - temp;
    let b: u8 = temp;
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
        .frame(Frame {
            fill: MENU_BG,
            inner_margin: Margin::same(0.0),
            outer_margin: Margin::same(0.0),
            ..default()
        });
    gui.show(egui_ctx.ctx_mut(), |ui| {
        ui.style_mut()
            .override_font_id = Some(FontId::new(
                20.0,
                 egui::FontFamily::Monospace
        ));
        ui.vertical_centered(|ui| {
            ui.add_space(SPACING * 0.1);
            ui.set_height(HEIGHT);
            ui.add_space(SPACING);
            ui.label("RBR-GUI");
            ui.label("Developed by");
            ui.hyperlink_to("Maj Guček", "https://github.com/MajGucek/RBR-GUI");
            ui.add_space(SPACING);
            let pedals = ui.button("Pedal Telemetry");
            let tires = ui.button("Tire Telemetry");
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
                let time = rbr.telemetry.get_time();
                println!("{}: {}: {}", time.hours, time.minutes, time.seconds);
                ui.label(format!("Race time: {}", time.seconds));
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
                next_state.set(DisplayState::Tires);
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