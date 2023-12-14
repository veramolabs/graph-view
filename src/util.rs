use bevy::math::Vec3;

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
