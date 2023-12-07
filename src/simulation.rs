use crate::assets::MyAssets;
use crate::identifiers::Identifier;
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
        .add_systems(Update, update_identifiers);
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
                    mesh: my_assets.mesh_handle.clone(),
                    // material: my_assets.material_handle.clone(),
                    material: my_assets.color_material_handle.clone(),
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
