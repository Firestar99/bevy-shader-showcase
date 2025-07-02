use crate::custom_shaders::f_discard::random_planes;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mesh = meshes.add(random_planes());

    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(materials.add(CustomMaterial {
            color: LinearRgba::WHITE,
            color_texture: Some(asset_server.load("textures/grass.png")),
        })),
        Transform::from_xyz(12.0, 0.5, 0.0),
    ));

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(materials.add(CustomMaterial {
            color: LinearRgba::WHITE,
            color_texture: Some(asset_server.load("textures/blending_transparent_window.png")),
        })),
        Transform::from_xyz(12.0, 2.5, 0.0),
    ));
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/g_blending.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
