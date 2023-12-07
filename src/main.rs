use bevy::input::common_conditions::input_toggle_active;
use bevy::pbr::ClusterConfig;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_panorbit_camera::*;
use bevy_window::PresentMode;
mod assets;
mod identifiers;
mod keyboard;
mod resources;
mod simulation;
mod touch;

use assets::AssetsPlugin;
use keyboard::KeyboardPlugin;
use resources::Configuration;
use simulation::SimulationPlugin;
use touch::TouchCameraPlugin;

fn main() {
    App::new()
        .init_resource::<Configuration>()
        .register_type::<Configuration>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync, // Reduces input lag.
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(AssetsPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(DefaultInspectorConfigPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(TouchCameraPlugin::default())
        .add_plugins(KeyboardPlugin)
        .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::W)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_plugins(SimulationPlugin)
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    //    mut meshes: ResMut<Assets<Mesh>>,
    //    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ClusterConfig::Single,
        PanOrbitCamera::default(),
    ));
}
