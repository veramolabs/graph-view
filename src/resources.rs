use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Configuration {
    pub identifiers: u32,
    pub container_size: f32,
    pub animation_duration: u64,
    pub connections: u32,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            identifiers: 10,
            container_size: 2.0,
            animation_duration: 2,
            connections: 0,
        }
    }
}
