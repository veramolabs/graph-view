use bevy::prelude::*;

#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct MyAssets {
    pub identifier_mesh_handle: Handle<Mesh>,
    pub identifier_material_handle: Handle<StandardMaterial>,
    pub identifier_selected_material_handle: Handle<StandardMaterial>,
    pub identifier_connected_material_handle: Handle<StandardMaterial>,
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
    my_assets.identifier_connected_material_handle = color_materials.add(StandardMaterial {
        base_color: Color::rgb(0.1, 0.1, 0.9),
        ..Default::default()
    });
    my_assets.identifier_selected_material_handle = color_materials.add(StandardMaterial {
        base_color: Color::RED,
        ..Default::default()
    });
    my_assets.identifier_material_handle = color_materials.add(StandardMaterial {
        base_color: Color::GOLD.with_a(0.6),
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });

    my_assets.connection_mesh_handle = meshes.add(Mesh::from(shape::Cylinder {
        radius: 0.02,
        height: 1.0,
        resolution: 5,
        segments: 1,
    }));
    my_assets.connection_material_handle = color_materials.add(StandardMaterial {
        base_color: Color::rgb(0.1, 0.1, 0.9).with_a(0.7),
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });
}
