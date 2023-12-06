use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Configuration {
    #[inspector(min = 0, max = 1000)]
    pub identifiers: u32,
    #[inspector(min = 1.0, max = 1000.0)]
    pub x: f32,
    #[inspector(min = 1.0, max = 1000.0)]
    pub y: f32,
    #[inspector(min = 1.0, max = 1000.0)]
    pub z: f32,
}

