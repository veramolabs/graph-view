use crate::assets::MyAssets;
use crate::events::{SelectRandomConnectedIdentifierEvent, SelectRandomIdentifierEvent};
use crate::identifiers::{Connection, Identifier};
use crate::resources::Configuration;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_easings::*;
use bevy_egui::EguiContext;
use bevy_inspector_egui::egui;
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_window::PrimaryWindow;
use rand::Rng;
use std::f64::consts::PI;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            inspector_ui.run_if(input_toggle_active(true, KeyCode::C)),
        )
        .add_systems(Update, update_identifiers.before(update_connections))
        .add_systems(Update, update_connections.after(update_identifiers));
    }
}

fn update_identifiers(
    mut commands: Commands,
    configuration: Res<Configuration>,
    query: Query<Entity, &Identifier>,
    my_assets: ResMut<MyAssets>,
) {
    if !configuration.is_changed() {
        return;
    };
    let current_count = query.iter().count() as u32;
    let target_count = configuration.identifiers;
    #[allow(clippy::comparison_chain)]
    if target_count > current_count {
        for _ in 0..(target_count - current_count) {
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
    } else if target_count < current_count {
        // Despawn excess cubes
        for entity in query.iter().take((current_count - target_count) as usize) {
            commands.entity(entity).despawn();
        }
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

fn update_connections(
    mut commands: Commands,
    configuration: Res<Configuration>,
    identifier_query: Query<(Entity, &Transform)>,
    connection_query: Query<Entity, &Connection>,
    my_assets: ResMut<MyAssets>,
) {
    if !configuration.is_changed() {
        return;
    };
    let mut rng = rand::thread_rng();
    let current_count = connection_query.iter().count() as u32;
    let target_count = configuration.connections;

    let identifier_count = identifier_query.iter().count() as u32;
    #[allow(clippy::comparison_chain)]
    if target_count > current_count {
        if identifier_count < 2 {
            return;
        }
        for _ in 0..(target_count - current_count) {
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
                    transform: Transform {
                        translation: mid_point,
                        rotation,
                        scale: Vec3::new(1.0, distance, 1.0),
                    },
                    visibility: Visibility::Hidden,
                    ..Default::default()
                },
                // Transform {
                //     translation: mid_point,
                //     rotation,
                //     scale: Vec3::new(1.0, distance, 1.0),
                // },
                // Transform::from_xyz(mid_point.x, mid_point.y, mid_point.z)
                //     .with_rotation(rotation)
                //     .with_scale(Vec3::new(1.0, distance, 1.0)),
                // Transform::from_xyz(mid_point.x, mid_point.y, mid_point.z)
                //     .with_rotation(rotation)
                //     .with_scale(Vec3::new(1.0, 0.00001, 1.0))
                //     .ease_to(
                //         Transform::from_xyz(mid_point.x, mid_point.y, mid_point.z)
                //             .with_rotation(rotation)
                //             .with_scale(Vec3::new(1.0, distance, 1.0)),
                //         bevy_easings::EaseFunction::QuadraticInOut,
                //         bevy_easings::EasingType::Once {
                //             duration: std::time::Duration::from_secs(
                //                 configuration.animation_duration,
                //             ),
                //         },
                //     ),
                Connection {
                    from: rnd1,
                    to: rnd2,
                },
            ));
        }
    } else if target_count < current_count {
        // Despawn excess cubes
        for entity in connection_query
            .iter()
            .take((current_count - target_count) as usize)
        {
            commands.entity(entity).despawn();
        }
    }
}

fn inspector_ui(
    mut commands: Commands,
    mut configuration: ResMut<Configuration>,
    query: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut camera_q: Query<(Entity, &Transform), With<PanOrbitCamera>>,
    mut ev_rnd_id: EventWriter<SelectRandomIdentifierEvent>,
    mut ev_rnd_c_id: EventWriter<SelectRandomConnectedIdentifierEvent>,
) {
    let mut egui_context = query.single().clone();

    egui::Window::new("Configuration")
        .vscroll(false)
        .hscroll(false)
        .default_width(250.0)
        .resizable(false)
        .show(egui_context.get_mut(), |ui| {
            // bevy_inspector_egui::bevy_inspector::ui_for_resource::<Configuration>(world, ui);
            ui.add(
                egui::Slider::new(&mut configuration.identifiers, 0..=100000).text("Identifiers"),
            );
            ui.add(
                egui::Slider::new(&mut configuration.connections, 0..=100000).text("Connections"),
            );
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
