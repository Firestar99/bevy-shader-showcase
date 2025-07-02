use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // white bevy
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(CustomMaterial {
            color: LinearRgba::WHITE,
            color_texture: Some(asset_server.load("textures/icon.png")),
        })),
        Transform::from_xyz(8.0, 0.5, 0.0),
    ));

    // blue bevy
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(CustomMaterial {
            color: LinearRgba::BLUE,
            color_texture: Some(asset_server.load("textures/icon.png")),
        })),
        Transform::from_xyz(8.0, 2.5, 0.0),
    ));

    // slice square
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(CustomMaterial {
            color: LinearRgba::WHITE,
            color_texture: Some(asset_server.load("textures/container2.png")),
        })),
        Transform::from_xyz(8.0, 4.5, 0.0),
    ));
}

// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/e_texture.wgsl".into()
    }
}
