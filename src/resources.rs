use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Configuration {
    #[inspector(min = 0, max = 10000)]
    pub identifiers: u32,
    #[inspector(min = 1.0, max = 1000.0)]
    pub container_size: f32,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            identifiers: 1000,
            container_size: 10.0,
        }
    }
}
