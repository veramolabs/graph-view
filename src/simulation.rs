use crate::assets::MyAssets;
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

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            inspector_ui.run_if(input_toggle_active(true, KeyCode::C)),
        )
        .add_systems(Update, update_identifiers)
        .add_systems(Update, update_connections);
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
    let mut rng = rand::thread_rng();
    let current_count = query.iter().count() as u32;
    let target_count = configuration.identifiers;
    #[allow(clippy::comparison_chain)]
    if target_count > current_count {
        // Spawn additional cubes
        for _ in 0..(target_count - current_count) {
            let x = rng.gen_range(-configuration.container_size..configuration.container_size);
            let y = rng.gen_range(-configuration.container_size..configuration.container_size);
            let z = rng.gen_range(-configuration.container_size..configuration.container_size);
            commands.spawn((
                MaterialMeshBundle {
                    // ... Mesh, Material, Transform
                    mesh: my_assets.identifier_mesh_handle.clone(),
                    // material: my_assets.material_handle.clone(),
                    material: my_assets.identifier_material_handle.clone(),
                    transform: Transform::from_xyz(x, y, z),

                    ..Default::default()
                },
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

fn update_connections(
    mut commands: Commands,
    configuration: Res<Configuration>,
    identifier_query: Query<(Entity, &Identifier, &Transform)>,
    connection_query: Query<Entity, &Connection>,
    my_assets: ResMut<MyAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
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
        for _ in 0..(target_count - current_count) {
            let (rnd1, identifier1, transform1) = identifier_query
                .iter()
                .nth(rng.gen_range(0..identifier_count as usize))
                .unwrap();
            let (rnd2, identifier2, transform2) = identifier_query
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
                    // ... Mesh, Material, Transform
                    // mesh: my_assets.connection_mesh_handle.clone(),
                    mesh: meshes.add(Mesh::from(shape::Capsule {
                        radius: 0.02,
                        depth: distance,
                        ..Default::default()
                    })),
                    // material: my_assets.material_handle.clone(),
                    material: my_assets.connection_material_handle.clone(),
                    transform: Transform::from_xyz(mid_point.x, mid_point.y, mid_point.z)
                        .with_rotation(rotation),

                    ..Default::default()
                },
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
        });
}
