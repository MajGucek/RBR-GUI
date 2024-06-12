// Hide Terminal
#![windows_subsystem = "windows"]

// helper.rs
mod helper;
use helper::*;

// udp_connection.rs
mod udp_connection;
use udp_connection::*;

// telemetry.rs
mod telemetry;
use telemetry::*;

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
