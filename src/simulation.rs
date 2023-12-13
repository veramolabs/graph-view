use crate::assets::MyAssets;
use crate::events::{
    AddConnectionsEvent, AddIdentifiersEvent, Forceatlas2Event, MoveIdentifiersRndEvent,
    SelectRandomConnectedIdentifierEvent, SelectRandomIdentifierEvent,
};
use crate::identifiers::{Connection, Identifier};
use crate::resources::Configuration;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_easings::*;
use bevy_egui::EguiContext;
use bevy_inspector_egui::egui;
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_window::PrimaryWindow;
use forceatlas2::*;
use rand::Rng;
use std::f64::consts::PI;
pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                inspector_ui.run_if(input_toggle_active(true, KeyCode::C)),
                simulation_ui,
            ),
        )
        .register_type::<Connection>()
        .add_systems(Update, add_connections)
        .add_systems(Update, move_identifiers_randomly)
        .add_systems(Update, move_identifiers_forceatlas2)
        .add_systems(Update, update_connections_transforms)
        .add_systems(Update, add_identifiers);
    }
}

fn random_point_in_sphere(radius: f32) -> (f32, f32, f32) {
    let mut rng = rand::thread_rng();
    let theta = rng.gen::<f32>() * 2.0 * PI as f32;
    let phi = rng.gen::<f32>() * PI as f32;
    let u = rng.gen::<f32>() * radius.powi(3);

    let r = u.cbrt();
    let x = r * phi.sin() * theta.cos();
    let y = r * phi.sin() * theta.sin();
    let z = r * phi.cos();

    (x, y, z)
}

fn inspector_ui(
    mut commands: Commands,
    mut configuration: ResMut<Configuration>,
    query: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut camera_q: Query<(Entity, &Transform), With<PanOrbitCamera>>,
    mut ev_rnd_id: EventWriter<SelectRandomIdentifierEvent>,
    mut ev_rnd_c_id: EventWriter<SelectRandomConnectedIdentifierEvent>,
    mut ev_move: EventWriter<MoveIdentifiersRndEvent>,
    mut ev_forceatlas2: EventWriter<Forceatlas2Event>,
) {
    let mut egui_context = query.single().clone();

    egui::Window::new("Configuration")
        .vscroll(false)
        .hscroll(false)
        .default_width(250.0)
        .resizable(false)
        .show(egui_context.get_mut(), |ui| {
            ui.add(egui::Slider::new(&mut configuration.container_size, 0.0..=100.0).text("Space"));
            ui.add(
                egui::Slider::new(&mut configuration.animation_duration, 1..=10)
                    .text("Duration (sec)"),
            );
        });

    egui::Window::new("Actions")
        .vscroll(false)
        .hscroll(false)
        .default_width(250.0)
        .resizable(false)
        .show(egui_context.get_mut(), |ui| {
            if ui.button("Select random identifier").clicked() {
                ev_rnd_id.send(SelectRandomIdentifierEvent);
            }
            if ui.button("Select random connected identifier").clicked() {
                ev_rnd_c_id.send(SelectRandomConnectedIdentifierEvent);
            }
            if ui.button("Move identifiers randomly").clicked() {
                ev_move.send(MoveIdentifiersRndEvent);
            }
            if ui.button("Move identifiers forceatlas2").clicked() {
                ev_forceatlas2.send(Forceatlas2Event(Settings {
                    ..Default::default()
                }));
            }
            ui.separator();
            if ui.button("Move camera randomly").clicked() {
                if let Ok((entity, transform)) = camera_q.get_single_mut() {
                    commands.entity(entity).insert(
                        transform.ease_to(
                            Transform::from_xyz(
                                rand::thread_rng().gen_range(
                                    -configuration.container_size..=configuration.container_size,
                                ),
                                rand::thread_rng().gen_range(
                                    -configuration.container_size..=configuration.container_size,
                                ),
                                rand::thread_rng().gen_range(
                                    -configuration.container_size..=configuration.container_size,
                                ),
                            )
                            .looking_at(Vec3::ZERO, Vec3::Y),
                            EaseFunction::QuarticInOut,
                            bevy_easings::EasingType::Once {
                                duration: (std::time::Duration::from_secs(
                                    configuration.animation_duration,
                                )),
                            },
                        ),
                    );
                };
            }

            if ui.button("Zoom out").clicked() {
                if let Ok((entity, transform)) = camera_q.get_single_mut() {
                    let new_position = transform.translation + configuration.container_size * 1.5;
                    commands.entity(entity).insert(
                        transform.ease_to(
                            Transform::from_xyz(new_position.x, new_position.y, new_position.z)
                                .looking_at(Vec3::ZERO, Vec3::Y),
                            EaseFunction::QuarticInOut,
                            bevy_easings::EasingType::Once {
                                duration: (std::time::Duration::from_secs(
                                    configuration.animation_duration,
                                )),
                            },
                        ),
                    );
                };
            }
        });
}

#[derive(Default)]
struct IdentiferCount {
    count: u32,
}

#[derive(Default)]
struct ConnectionsCount {
    count: u32,
}

fn simulation_ui(
    query: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut id_count: Local<IdentiferCount>,
    mut conn_count: Local<ConnectionsCount>,
    mut ev_id: EventWriter<AddIdentifiersEvent>,
    mut ev_conn: EventWriter<AddConnectionsEvent>,
) {
    let mut egui_context = query.single().clone();

    egui::Window::new("Simulation")
        .vscroll(false)
        .hscroll(false)
        .default_width(250.0)
        .resizable(false)
        .show(egui_context.get_mut(), |ui| {
            ui.add(egui::Slider::new(&mut id_count.count, 0..=100000).text("Identifiers"));
            if ui.button("Add").clicked() {
                ev_id.send(AddIdentifiersEvent {
                    count: id_count.count,
                });
            }
            ui.separator();
            ui.add(egui::Slider::new(&mut conn_count.count, 0..=100000).text("Connections"));
            if ui.button("Add").clicked() {
                ev_conn.send(AddConnectionsEvent {
                    count: conn_count.count,
                });
            }
        });
}

fn add_identifiers(
    mut commands: Commands,
    mut ev: EventReader<AddIdentifiersEvent>,
    configuration: Res<Configuration>,
    my_assets: ResMut<MyAssets>,
) {
    for e in ev.read() {
        for _ in 0..e.count {
            let (x, y, z) = random_point_in_sphere(configuration.container_size);
            commands.spawn((
                MaterialMeshBundle {
                    mesh: my_assets.identifier_mesh_handle.clone(),
                    material: my_assets.identifier_material_handle.clone(),
                    ..Default::default()
                },
                Transform::from_xyz(x, y, z)
                    .with_scale(Vec3::new(0.0001, 0.0001, 0.0001))
                    .ease_to(
                        Transform::from_xyz(x, y, z).with_scale(Vec3::new(0.5, 0.5, 0.5)),
                        bevy_easings::EaseFunction::QuadraticOut,
                        bevy_easings::EasingType::Once {
                            duration: std::time::Duration::from_secs(
                                configuration.animation_duration,
                            ),
                        },
                    ),
                Identifier {},
            ));
        }
    }
}

fn add_connections(
    mut commands: Commands,
    mut ev: EventReader<AddConnectionsEvent>,
    configuration: Res<Configuration>,
    identifier_query: Query<(Entity, &Transform), With<Identifier>>,
    my_assets: ResMut<MyAssets>,
) {
    for e in ev.read() {
        let mut rng = rand::thread_rng();
        let identifier_count = identifier_query.iter().count() as u32;

        for _ in 0..e.count {
            let (rnd1, transform1) = identifier_query
                .iter()
                .nth(rng.gen_range(0..identifier_count as usize))
                .unwrap();
            let (rnd2, transform2) = identifier_query
                .iter()
                .nth(rng.gen_range(0..identifier_count as usize))
                .unwrap();

            let mid_point = transform1.translation.lerp(transform2.translation, 0.5);
            let distance = transform1.translation.distance(transform2.translation);
            let rotation = Quat::from_rotation_arc(
                Vec3::Y,
                (transform2.translation - transform1.translation).normalize(),
            );

            commands.spawn((
                MaterialMeshBundle {
                    mesh: my_assets.connection_mesh_handle.clone(),
                    material: my_assets.connection_material_handle.clone(),
                    // transform: Transform {
                    //     translation: mid_point,
                    //     rotation,
                    //     scale: Vec3::new(1.0, distance, 1.0),
                    // },
                    visibility: Visibility::Visible,
                    ..Default::default()
                },
                Transform::from_xyz(mid_point.x, mid_point.y, mid_point.z)
                    .with_rotation(rotation)
                    .with_scale(Vec3::new(1.0, 0.00001, 1.0))
                    .ease_to(
                        Transform::from_xyz(mid_point.x, mid_point.y, mid_point.z)
                            .with_rotation(rotation)
                            .with_scale(Vec3::new(1.0, distance, 1.0)),
                        bevy_easings::EaseFunction::QuadraticInOut,
                        bevy_easings::EasingType::Once {
                            duration: std::time::Duration::from_secs(
                                configuration.animation_duration,
                            ),
                        },
                    ),
                Connection {
                    from: rnd1,
                    to: rnd2,
                },
            ));
        }
    }
}

fn move_identifiers_randomly(
    mut commands: Commands,
    mut ev: EventReader<MoveIdentifiersRndEvent>,
    configuration: Res<Configuration>,
    identifier_query: Query<(Entity, &Transform), With<Identifier>>,
) {
    for _ in ev.read() {
        for (entity, transform) in identifier_query.iter() {
            let (x, y, z) = random_point_in_sphere(configuration.container_size);
            commands.entity(entity).insert(transform.ease_to(
                Transform::from_xyz(x, y, z),
                EaseFunction::QuarticOut,
                bevy_easings::EasingType::Once {
                    duration: (std::time::Duration::from_secs(configuration.animation_duration)),
                },
            ));
        }
    }
}

fn update_connections_transforms(
    mut conn_query: Query<(&mut Transform, &Connection), (With<Connection>, Without<Identifier>)>,
    id_query: Query<&Transform, (With<Identifier>, Without<Connection>)>,
) {
    for (mut transform, connection) in conn_query.iter_mut() {
        if let Ok(from_transform) = id_query.get(connection.from) {
            if let Ok(to_transform) = id_query.get(connection.to) {
                let mid_point = from_transform
                    .translation
                    .lerp(to_transform.translation, 0.5);
                let distance = from_transform
                    .translation
                    .distance(to_transform.translation);
                let rotation = Quat::from_rotation_arc(
                    Vec3::Y,
                    (to_transform.translation - from_transform.translation).normalize(),
                );

                *transform = Transform::from_xyz(mid_point.x, mid_point.y, mid_point.z)
                    .with_rotation(rotation)
                    .with_scale(Vec3::new(1.0, distance, 1.0));
            }
        }
    }
}

fn move_identifiers_forceatlas2(
    mut commands: Commands,
    mut ev: EventReader<Forceatlas2Event>,
    configuration: Res<Configuration>,
    identifier_query: Query<(Entity, &Transform), With<Identifier>>,
    conn_query: Query<&Connection, With<Connection>>,
) {
    for settings in ev.read() {
        // for (entity, transform) in conn_query.iter() {
        //     let (x, y, z) = random_point_in_sphere(2.0);
        //     commands.entity(entity).insert(transform.ease_to(
        //         Transform::from_xyz(x, y, z),
        //         EaseFunction::QuarticOut,
        //         bevy_easings::EasingType::Once {
        //             duration: (std::time::Duration::from_secs(configuration.animation_duration)),
        //         },
        //     ));
        // }
        let mut rng = rand::thread_rng();
        const EDGES: usize = 80_000;
        const NODES: usize = 10_000;
        const ITERATIONS: u32 = 10;

        eprintln!("Generating graph...");
        let edges = conn_query
            .iter()
            .map(|connection| {
                (
                    connection.from.index() as usize,
                    connection.to.index() as usize,
                )
            })
            .collect();

        println!("{:?}", edges);

        let identier_count = identifier_query.iter().count() as usize;
        let mut layout = Layout::<f32>::from_graph(
            edges,
            Nodes::Degree(identier_count),
            None,
            None,
            Settings {
                #[cfg(feature = "barnes_hut")]
                barnes_hut: None,
                chunk_size: Some(256),
                dimensions: 3,
                dissuade_hubs: false,
                ka: 0.5,
                kg: 1.0,
                kr: 0.1,
                lin_log: false,
                prevent_overlapping: None,
                speed: 1.0,
                strong_gravity: false,
            },
        );

        eprintln!("Computing layout...");
        for i in 0..ITERATIONS {
            println!("{}/{}", i, ITERATIONS);
            layout.iteration();
        }

        for (h1, h2) in layout.edges.iter() {
            println!("{:?} {:?}", h1, h2);
            if let Ok((entity, transform)) = identifier_query.get(Entity::from_raw(*h1 as u32)) {
                let pos = layout.points.get(*h1);
                println!("{:?}", pos);
                commands.entity(entity).insert(transform.ease_to(
                    Transform::from_xyz(pos[0], pos[1], pos[2]),
                    EaseFunction::QuarticOut,
                    bevy_easings::EasingType::Once {
                        duration: (std::time::Duration::from_secs(
                            configuration.animation_duration,
                        )),
                    },
                ));
            }

            if let Ok((entity, transform)) = identifier_query.get(Entity::from_raw(*h2 as u32)) {
                let pos = layout.points.get(*h2);
                println!("{:?}", pos);
                commands.entity(entity).insert(transform.ease_to(
                    Transform::from_xyz(pos[0], pos[1], pos[2]),
                    EaseFunction::QuarticOut,
                    bevy_easings::EasingType::Once {
                        duration: (std::time::Duration::from_secs(
                            configuration.animation_duration,
                        )),
                    },
                ));
            }
        }
    }
}
