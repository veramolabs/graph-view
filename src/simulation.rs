use crate::assets::MyAssets;
use crate::events::*;
use crate::identifiers::{Connection, Identifier};
use crate::resources::Configuration;
use crate::util::random_point_in_sphere;
use bevy::prelude::*;
use bevy_easings::*;
use bevy_egui::EguiContext;
use bevy_inspector_egui::egui;
use bevy_mod_picking::prelude::*;
use bevy_mod_picking::PickableBundle;
use bevy_window::PrimaryWindow;
use forceatlas2::*;
use graph::page_rank::PageRankConfig;
use graph::prelude::*;
use rand::Rng;
use std::collections::HashSet;
pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_systems(
            //     Update,
            //     (
            //         inspector_ui.run_if(input_toggle_active(true, KeyCode::C)),
            //         simulation_ui,
            //         force_atlas_ui,
            //     ),
            // )
            .register_type::<Connection>()
            .add_systems(Update, add_connections)
            .add_systems(Update, move_identifiers_randomly)
            .add_systems(Update, move_identifiers_forceatlas2)
            .add_systems(Update, resize_identifiers_pagerank)
            .add_systems(Update, update_connections_transforms)
            .add_systems(Update, add_identifiers);
    }
}

pub struct IdentiferCount {
    count: u32,
}
impl Default for IdentiferCount {
    fn default() -> Self {
        Self { count: 100 }
    }
}

pub struct ConnectionsCount {
    count: u32,
}
impl Default for ConnectionsCount {
    fn default() -> Self {
        Self { count: 100 }
    }
}

pub fn simulation_ui(
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

pub struct Gravity(f32);
impl Default for Gravity {
    fn default() -> Self {
        Self(0.3)
    }
}

pub struct Atrraction(f32);
impl Default for Atrraction {
    fn default() -> Self {
        Self(0.9)
    }
}

pub struct Repulsion(f32);
impl Default for Repulsion {
    fn default() -> Self {
        Self(0.05)
    }
}

pub struct Iterations(u32);
impl Default for Iterations {
    fn default() -> Self {
        Self(100)
    }
}

pub fn force_atlas_ui(
    query: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut gravity: Local<Gravity>,
    mut attraction: Local<Atrraction>,
    mut repulsion: Local<Repulsion>,
    mut iterations: Local<Iterations>,
    mut ev: EventWriter<Forceatlas2Event>,
) {
    let mut egui_context = query.single().clone();

    egui::Window::new("ForceAtlas2")
        .vscroll(false)
        .hscroll(false)
        .default_width(250.0)
        .resizable(false)
        .show(egui_context.get_mut(), |ui| {
            ui.add(egui::Slider::new(&mut gravity.0, 0.0..=2.0).text("Gravity"));
            ui.add(egui::Slider::new(&mut attraction.0, 0.0..=2.0).text("Attraction"));
            ui.add(egui::Slider::new(&mut repulsion.0, 0.0..=2.0).text("Repulsion"));
            ui.add(egui::Slider::new(&mut iterations.0, 1..=1000).text("Iterations"));
            if ui.button("Move identifiers forceatlas2").clicked() {
                ev.send(Forceatlas2Event {
                    settings: Settings {
                        kg: gravity.0,
                        ka: attraction.0,
                        kr: repulsion.0,
                        ..Default::default()
                    },
                    iterations: iterations.0,
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
                PickableBundle::default(),
                On::<Pointer<Click>>::run(
                    |event: Listener<Pointer<Click>>,
                     mut ev: EventWriter<SelectIdentifierEvent>| {
                        info!("The pointer clicked entity {:?}", event.target);
                        ev.send(SelectIdentifierEvent(event.target));
                    },
                ),
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

            if rnd1 == rnd2 {
                continue;
            }

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
        // eprintln!("Generating graph...");
        let edges: Vec<(usize, usize)> = conn_query
            .iter()
            .map(|connection| {
                (
                    connection.from.index() as usize,
                    connection.to.index() as usize,
                )
            })
            .collect();

        // count the number of unique nodes
        let flattened: Vec<usize> = edges
            .clone()
            .into_iter()
            .flat_map(|(a, b)| vec![a, b])
            .collect();
        let unique_values: HashSet<_> = flattened.into_iter().collect();

        // get largest value
        let max = unique_values.iter().max().unwrap();
        let mut layout = Layout::<f32>::from_graph(
            edges,
            Nodes::Degree(*max + 1),
            None,
            None,
            Settings {
                #[cfg(feature = "barnes_hut")]
                barnes_hut: None,
                chunk_size: Some(256),
                dimensions: 3,
                dissuade_hubs: true,
                ka: settings.settings.ka,
                kg: settings.settings.kg,
                kr: settings.settings.kr,
                lin_log: false,
                prevent_overlapping: None,
                speed: 1.0,
                strong_gravity: true,
            },
        );

        // eprintln!("Computing layout...");
        for _ in 0..settings.iterations {
            // println!("{}/{}", i, ITERATIONS);
            layout.iteration();
        }

        for (h1, h2) in layout.edges.iter() {
            if let Ok((entity, transform)) = identifier_query.get(Entity::from_raw(*h1 as u32)) {
                let pos = layout.points.get(*h1);
                commands.entity(entity).insert(transform.ease_to(
                    Transform::from_xyz(pos[0], pos[1], pos[2]).with_scale(transform.scale),
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
                commands.entity(entity).insert(transform.ease_to(
                    Transform::from_xyz(pos[0], pos[1], pos[2]).with_scale(transform.scale),
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

fn resize_identifiers_pagerank(
    mut commands: Commands,
    mut ev: EventReader<PageRankEvent>,
    configuration: Res<Configuration>,
    identifier_query: Query<(Entity, &Transform), With<Identifier>>,
    conn_query: Query<&Connection, With<Connection>>,
) {
    for settings in ev.read() {
        // eprintln!("Generating graph...");

        let edges: Vec<(usize, usize)> = conn_query
            .iter()
            .map(|connection| {
                (
                    connection.from.index() as usize,
                    connection.to.index() as usize,
                )
            })
            .collect();

        // // count the number of unique nodes
        // let flattened: Vec<usize> = edges
        //     .clone()
        //     .into_iter()
        //     .flat_map(|(a, b)| vec![a, b])
        //     .collect();

        // let unique_values: HashSet<_> = flattened.into_iter().collect();
        // unique_values.iter().enumerate().for_each(|(i, v)| {
        //     println!("{}: {}", i, v);
        // });

        let (unique_values, indexed_vec) = process_vec(&edges);

        // println!("Unique Values: {:?}", unique_values);
        // println!("Indexed Vec: {:?}", indexed_vec);

        let graph: DirectedCsrGraph<usize> = GraphBuilder::new().edges(indexed_vec).build();

        let (ranks, _, _) = page_rank(&graph, settings.config);
        // println!("Ranks: {:?}", ranks);
        let mut cloned_ranks = ranks.clone();
        normalize(&mut cloned_ranks);

        // println!("Normalized Ranks: {:?}", cloned_ranks);
        for (i, rank) in cloned_ranks.iter().enumerate() {
            if let Ok((entity, transform)) =
                identifier_query.get(Entity::from_raw(unique_values[i] as u32))
            {
                let pos = transform.translation;
                commands.entity(entity).insert(
                    transform.ease_to(
                        Transform::from_xyz(pos.x, pos.y, pos.z)
                            .with_scale(Vec3::ONE * (*rank * 3.0 + 0.5)),
                        EaseFunction::QuarticOut,
                        bevy_easings::EasingType::Once {
                            duration: (std::time::Duration::from_secs(
                                configuration.animation_duration,
                            )),
                        },
                    ),
                );
            }
        }

        // for (h1, h2) in layout.edges.iter() {
        //     if let Ok((entity, transform)) = identifier_query.get(Entity::from_raw(*h1 as u32)) {
        //         let pos = layout.points.get(*h1);
        //         commands.entity(entity).insert(transform.ease_to(
        //             Transform::from_xyz(pos[0], pos[1], pos[2]),
        //             EaseFunction::QuarticOut,
        //             bevy_easings::EasingType::Once {
        //                 duration: (std::time::Duration::from_secs(
        //                     configuration.animation_duration,
        //                 )),
        //             },
        //         ));
        //     }

        //     if let Ok((entity, transform)) = identifier_query.get(Entity::from_raw(*h2 as u32)) {
        //         let pos = layout.points.get(*h2);
        //         commands.entity(entity).insert(transform.ease_to(
        //             Transform::from_xyz(pos[0], pos[1], pos[2]),
        //             EaseFunction::QuarticOut,
        //             bevy_easings::EasingType::Once {
        //                 duration: (std::time::Duration::from_secs(
        //                     configuration.animation_duration,
        //                 )),
        //             },
        //         ));
        //     }
        // }
    }
}

fn normalize(vec: &mut [f32]) {
    let min = vec.iter().cloned().fold(f32::INFINITY, f32::min);
    let max = vec.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

    if max - min != 0.0 {
        vec.iter_mut().for_each(|x| *x = (*x - min) / (max - min));
    }
}

fn process_vec(input: &[(usize, usize)]) -> (Vec<usize>, Vec<(usize, usize)>) {
    // Step 1: Extract and sort unique values
    let mut unique_values: Vec<usize> = input.iter().flat_map(|(a, b)| vec![*a, *b]).collect();
    unique_values.sort_unstable();
    unique_values.dedup();

    // Step 2: Create a map for value to index
    let value_to_index: std::collections::HashMap<_, _> = unique_values
        .iter()
        .enumerate()
        .map(|(index, &value)| (value, index))
        .collect();

    // Step 3: Transform the original vector
    let indexed_vec: Vec<(usize, usize)> = input
        .iter()
        .map(|(a, b)| (value_to_index[&a], value_to_index[&b]))
        .collect();

    (unique_values, indexed_vec)
}

pub struct PageRankIterations(usize);
impl Default for PageRankIterations {
    fn default() -> Self {
        Self(20)
    }
}
pub struct PageRankTolerance(f64);
impl Default for PageRankTolerance {
    fn default() -> Self {
        Self(1.0E-4f64)
    }
}

pub struct PageRankDamping(f32);
impl Default for PageRankDamping {
    fn default() -> Self {
        Self(0.8500f32)
    }
}

pub fn page_rank_ui(
    query: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut iterations: Local<PageRankIterations>,
    mut tolerance: Local<PageRankTolerance>,
    mut damping: Local<PageRankDamping>,
    mut ev: EventWriter<PageRankEvent>,
) {
    let mut egui_context = query.single().clone();

    egui::Window::new("Page Rank")
        .vscroll(false)
        .hscroll(false)
        .default_width(250.0)
        .resizable(false)
        .show(egui_context.get_mut(), |ui| {
            ui.add(egui::Slider::new(&mut iterations.0, 1..=100).text("Iterations"));
            ui.add(egui::Slider::new(&mut damping.0, 0.0..=2.0).text("Damping"));
            ui.add(egui::Slider::new(&mut tolerance.0, 0.0..=0.001).text("Tolerance"));
            if ui.button("Resize").clicked() {
                ev.send(PageRankEvent {
                    config: PageRankConfig {
                        max_iterations: iterations.0,
                        tolerance: tolerance.0,
                        damping_factor: damping.0,
                    },
                });
            }
        });
}
