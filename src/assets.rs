use bevy::prelude::*;
use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    reflect::TypePath,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
};

#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct MyAssets {
    pub mesh_handle: Handle<Mesh>,
    pub material_handle: Handle<CustomMaterial>,
    pub color_material_handle: Handle<ColorMaterial>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyAssets>()
            .register_type::<MyAssets>()
            .add_plugins((MaterialPlugin::<CustomMaterial>::default(),))
            .add_systems(Startup, setup);
    }
}

fn setup(
    mut my_assets: ResMut<MyAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    // my_assets.mesh_handle = meshes.add(Mesh::try_from(shape::Cube { size: 0.1 }).unwrap());
    my_assets.mesh_handle = meshes.add(
        Mesh::try_from(shape::Icosphere {
            radius: 0.1,
            subdivisions: 2,
        })
        .unwrap(),
    );
    my_assets.material_handle = materials.add(CustomMaterial {
        color: Color::GREEN,
        is_red: true,
    });
    my_assets.color_material_handle = color_materials.add(ColorMaterial {
        color: Color::GREEN,
        texture: None,
    });
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/shader_defs.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if key.bind_group_data.is_red {
            let fragment = descriptor.fragment.as_mut().unwrap();
            fragment.shader_defs.push("IS_RED".into());
        }
        Ok(())
    }
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
#[bind_group_data(CustomMaterialKey)]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    is_red: bool,
}

// This key is used to identify a specific permutation of this material pipeline.
// In this case, we specialize on whether or not to configure the "IS_RED" shader def.
// Specialization keys should be kept as small / cheap to hash as possible,
// as they will be used to look up the pipeline for each drawn entity with this material type.
#[derive(Eq, PartialEq, Hash, Clone)]
pub struct CustomMaterialKey {
    is_red: bool,
}

impl From<&CustomMaterial> for CustomMaterialKey {
    fn from(material: &CustomMaterial) -> Self {
        Self {
            is_red: material.is_red,
        }
    }
}
