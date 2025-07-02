use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    let all_meshes = [
        Cuboid::default().mesh().build(),
        Sphere::default().mesh().build(),
        Torus::default().mesh().build(),
        Cone::default().mesh().build(),
    ];

    let mut spawn = |mesh: Mesh, offset: Vec3| {
        commands.spawn((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(CustomMaterial {})),
            Transform::from_translation(offset),
        ));
    };

    for (i, mesh) in all_meshes.into_iter().enumerate() {
        spawn(mesh, vec3(14.0, 2.0 * i as f32 + 0.5, 0.));
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/h_normals.wgsl".into()
    }
}
