use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut discard_materials: ResMut<Assets<DiscardMaterial>>,
    mut blend_materials: ResMut<Assets<BlendMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(discard_materials.add(DiscardMaterial {
            color: LinearRgba::WHITE,
            color_texture: Some(asset_server.load("textures/icon.png")),
        })),
        Transform::from_xyz(15.0, 0.5, 0.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(blend_materials.add(BlendMaterial {
            color: LinearRgba::WHITE,
            color_texture: Some(asset_server.load("textures/icon.png")),
        })),
        Transform::from_xyz(15.0, 0.5, -3.0),
    ));
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct DiscardMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

impl Material for DiscardMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/f_discard.wgsl".into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BlendMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

impl Material for BlendMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/e_texture.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
