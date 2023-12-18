use bevy::input::common_conditions::input_toggle_active;
use bevy::pbr::ClusterConfig;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_easings::EasingsPlugin;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_panorbit_camera::*;
mod assets;
mod events;
mod identifiers;
mod keyboard;
mod resources;
mod simulation;
mod touch;
mod ui;
mod util;

use assets::AssetsPlugin;
use bevy_window::PresentMode;
use events::EventsPlugin;
use identifiers::IdentifiersPlugin;
use keyboard::KeyboardPlugin;
use resources::Configuration;
use simulation::SimulationPlugin;
use touch::TouchCameraPlugin;
use ui::UiPlugin;

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
        .add_plugins(UiPlugin)
        .add_plugins(EasingsPlugin)
        .add_plugins(AssetsPlugin)
        .add_plugins(EventsPlugin)
        .add_plugins(IdentifiersPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(DefaultInspectorConfigPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(TouchCameraPlugin::default())
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(KeyboardPlugin)
        .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::W)))
        .add_plugins(SimulationPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let initial_camera_location = Vec3::new(-2.0, 2.5, 5.0);

    commands.spawn((
        Camera3dBundle {
            projection: OrthographicProjection {
                near: -500.0,
                far: 500.0,
                scale: 12.5,
                scaling_mode: ScalingMode::FixedVertical(0.8),
                ..default()
            }
            .into(),

            transform: Transform::from_translation(initial_camera_location)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ClusterConfig::Single,
        PanOrbitCamera::default(),
    ));

    // cube
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::rgb_u8(124, 144, 255).into()),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     ..default()
    // });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });
}
