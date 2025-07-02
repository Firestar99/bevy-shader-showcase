use crate::double_sided::DoubleSidedExt;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

pub fn random_planes() -> Mesh {
    let global_offset = vec3(0.5, 0., 0.5);
    let translations = &[
        vec3(1., 0., 0.1),
        vec3(0.2, 0., 0.8),
        vec3(0.4, 0., 0.2),
        vec3(0.9, 0., 0.4),
        vec3(0.3, 0., 0.6),
    ];
    translations
        .iter()
        .map(|v| {
            Plane3d {
                normal: Dir3::Z,
                half_size: Vec2::splat(0.5),
            }
            .mesh()
            .build()
            .translated_by(*v - global_offset)
            .with_double_sided()
        })
        .reduce(|mut a, b| {
            a.merge(&b).unwrap();
            a
        })
        .unwrap()
}

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
        Transform::from_xyz(10.0, 0.5, 0.0),
    ));

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(materials.add(CustomMaterial {
            color: LinearRgba::WHITE,
            color_texture: Some(asset_server.load("textures/blending_transparent_window.png")),
        })),
        Transform::from_xyz(10.0, 2.5, 0.0),
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
        "shaders/f_discard.wgsl".into()
    }
}
