use crate::learn_opengl_pbr_materials::all_materials;
use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, AnimatedMaterialExtension>>>,
    asset_server: Res<AssetServer>,
) {
    let offset = vec3(38.0, 0.5, 0.0);
    for (i, base) in all_materials(&asset_server).into_iter().enumerate() {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.75))),
            MeshMaterial3d(materials.add(ExtendedMaterial {
                base: base.clone(),
                extension: AnimatedMaterialExtension {},
            })),
            Transform::from_translation(offset + vec3(2. * i as f32, 0., 0.)),
        ));
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct AnimatedMaterialExtension {}

impl MaterialExtension for AnimatedMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        "shaders/n_pbr_animated.wgsl".into()
    }
}
