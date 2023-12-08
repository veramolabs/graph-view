use bevy::prelude::*;

#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct MyAssets {
    pub identifier_mesh_handle: Handle<Mesh>,
    pub identifier_material_handle: Handle<StandardMaterial>,
    pub connection_mesh_handle: Handle<Mesh>,
    pub connection_material_handle: Handle<StandardMaterial>,
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
    my_assets.identifier_mesh_handle = meshes.add(
        Mesh::try_from(shape::Icosphere {
            radius: 0.1,
            subdivisions: 2,
        })
        .unwrap(),
    );
    my_assets.identifier_material_handle = color_materials.add(StandardMaterial {
        emissive: Color::GOLD,
        ..Default::default()
    });

    my_assets.connection_mesh_handle = meshes.add(Mesh::from(shape::Capsule {
        radius: 0.05,
        depth: 3.0,
        ..Default::default()
    }));
    my_assets.connection_material_handle = color_materials.add(StandardMaterial {
        emissive: Color::BLUE.with_a(0.5),
        ..Default::default()
    });
}
