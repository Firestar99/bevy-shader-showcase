use crate::learn_opengl_pbr_materials::all_materials;
use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, CellShadedMaterialExtension>>>,
    asset_server: Res<AssetServer>,
) {
    let offset = vec3(36.0, 2.5, 0.0);
    for (i, base) in all_materials(&asset_server).into_iter().enumerate() {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::default())),
            MeshMaterial3d(materials.add(ExtendedMaterial {
                base: base.clone(),
                extension: CellShadedMaterialExtension {},
            })),
            Transform::from_translation(offset + vec3(2. * i as f32, 0., 0.)),
        ));
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct CellShadedMaterialExtension {}

impl MaterialExtension for CellShadedMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        "shaders/o_pbr_cell_shaded.wgsl".into()
    }
}
