use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Configuration {
    pub container_size: f32,
    pub animation_duration: u64,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            container_size: 4.0,
            animation_duration: 2,
        }
    }
}
