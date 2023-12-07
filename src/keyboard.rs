use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
pub struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard_controls);
    }
}

fn keyboard_controls(
    time: Res<Time>,
    key_input: Res<Input<KeyCode>>,
    mut pan_orbit_query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    for (mut pan_orbit, mut transform) in pan_orbit_query.iter_mut() {
        if key_input.pressed(KeyCode::ControlLeft) {
            // Jump focus point 1m using Ctrl+Shift + Arrows
            if key_input.pressed(KeyCode::ShiftLeft) {
                if key_input.just_pressed(KeyCode::Right) {
                    pan_orbit.target_focus += Vec3::X;
                }
                if key_input.just_pressed(KeyCode::Left) {
                    pan_orbit.target_focus -= Vec3::X;
                }
                if key_input.just_pressed(KeyCode::Up) {
                    pan_orbit.target_focus += Vec3::Y;
                }
                if key_input.just_pressed(KeyCode::Down) {
                    pan_orbit.target_focus -= Vec3::Y;
                }
            } else {
                // Jump by 45 degrees using Left Ctrl + Arrows
                if key_input.just_pressed(KeyCode::Right) {
                    pan_orbit.target_alpha += 45f32.to_radians();
                }
                if key_input.just_pressed(KeyCode::Left) {
                    pan_orbit.target_alpha -= 45f32.to_radians();
                }
                if key_input.just_pressed(KeyCode::Up) {
                    pan_orbit.target_beta += 45f32.to_radians();
                }
                if key_input.just_pressed(KeyCode::Down) {
                    pan_orbit.target_beta -= 45f32.to_radians();
                }
            }
        }
        // Pan using Left Shift + Arrows
        else if key_input.pressed(KeyCode::ShiftLeft) {
            let mut delta_translation = Vec3::ZERO;
            if key_input.pressed(KeyCode::Right) {
                delta_translation += transform.rotation * Vec3::X * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::Left) {
                delta_translation += transform.rotation * Vec3::NEG_X * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::Up) {
                delta_translation += transform.rotation * Vec3::Y * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::Down) {
                delta_translation += transform.rotation * Vec3::NEG_Y * time.delta_seconds();
            }
            transform.translation += delta_translation;
            pan_orbit.target_focus += delta_translation;
        }
        // Smooth rotation using arrow keys without modifier
        else {
            if key_input.pressed(KeyCode::Right) {
                pan_orbit.target_alpha += 50f32.to_radians() * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::Left) {
                pan_orbit.target_alpha -= 50f32.to_radians() * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::Up) {
                pan_orbit.target_beta += 50f32.to_radians() * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::Down) {
                pan_orbit.target_beta -= 50f32.to_radians() * time.delta_seconds();
            }

            // Zoom with Z and X
            if key_input.pressed(KeyCode::Z) {
                pan_orbit.radius = pan_orbit
                    .radius
                    .map(|radius| radius - 5.0 * time.delta_seconds());
            }
            if key_input.pressed(KeyCode::X) {
                pan_orbit.radius = pan_orbit
                    .radius
                    .map(|radius| radius + 5.0 * time.delta_seconds());
            }
        }

        // Force camera to update its transform
        pan_orbit.force_update = true;
    }
}
