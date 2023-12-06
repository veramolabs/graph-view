use crate::identifiers::Identifier;
use crate::resources::Configuration;
use bevy::prelude::*;
use bevy_egui::EguiContext;
use bevy_inspector_egui::egui;
use bevy_window::PrimaryWindow;
use rand::Rng;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, inspector_ui)
            .add_systems(Update, update_identifiers);
    }
}

fn update_identifiers(
    mut commands: Commands,
    configuration: Res<Configuration>,
    query: Query<Entity, &Identifier>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !configuration.is_changed() {
        return;
    };
    let mut rng = rand::thread_rng();
    let current_count = query.iter().count() as u32;
    let target_count = configuration.identifiers;

    if target_count > current_count {
        // Spawn additional cubes
        for _ in 0..(target_count - current_count) {
            let x = rng.gen_range(0.0..configuration.x);
            let y = rng.gen_range(0.0..configuration.y);
            let z = rng.gen_range(0.0..configuration.z);
            commands.spawn((
                PbrBundle {
                    // ... Mesh, Material, Transform
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                    material: materials.add(Color::rgb(0.18, 0.7, 0.6).into()),
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

fn inspector_ui(world: &mut World, mut disabled: Local<bool>) {
    let space_pressed = world
        .resource::<Input<KeyCode>>()
        .just_pressed(KeyCode::Space);
    if space_pressed {
        *disabled = !*disabled;
    }
    if *disabled {
        return;
    }

    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    egui::Window::new("Configuration").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            bevy_inspector_egui::bevy_inspector::ui_for_resource::<Configuration>(world, ui);

            ui.separator();
            ui.label("Press space to toggle");
        });
    });
}
