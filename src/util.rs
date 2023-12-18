use bevy::math::Vec3;
use rand::Rng;
use std::f64::consts::PI;

pub fn calculate_from_translation_and_focus(translation: Vec3, focus: Vec3) -> (f32, f32, f32) {
    let comp_vec = translation - focus;
    let mut radius = comp_vec.length();
    if radius == 0.0 {
        radius = 0.05; // Radius 0 causes problems
    }
    let alpha = if comp_vec.x == 0.0 && comp_vec.z >= 0.0 {
        0.0
    } else {
        (comp_vec.z / (comp_vec.x.powi(2) + comp_vec.z.powi(2)).sqrt()).acos()
    };
    let beta = (comp_vec.y / radius).asin();
    (alpha, beta, radius)
}

pub fn random_point_in_sphere(radius: f32) -> (f32, f32, f32) {
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
