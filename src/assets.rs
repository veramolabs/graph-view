use bevy::prelude::*;

#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct MyAssets {
    pub mesh_handle: Handle<Mesh>,
    pub color_material_handle: Handle<StandardMaterial>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyAssets>()
            .register_type::<MyAssets>()
            .add_systems(Startup, setup);
    }
}

fn setup(
    mut my_assets: ResMut<MyAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<StandardMaterial>>,
) {
    // my_assets.mesh_handle = meshes.add(Mesh::try_from(shape::Cube { size: 0.1 }).unwrap());
    my_assets.mesh_handle = meshes.add(
        Mesh::try_from(shape::Icosphere {
            radius: 0.1,
            subdivisions: 2,
        })
        .unwrap(),
    );
    my_assets.color_material_handle = color_materials.add(StandardMaterial {
        emissive: Color::GOLD,
        ..Default::default()
    });
}
