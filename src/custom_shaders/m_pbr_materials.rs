use crate::learn_opengl_pbr_materials;
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mut spawn_sphere = |mat: StandardMaterial, offset: Vec3| {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.75))),
            MeshMaterial3d(materials.add(mat)),
            Transform::from_translation(offset),
        ));
    };

    let offset = vec3(26.0, 0.5, 0.0);
    for (i, base) in learn_opengl_pbr_materials::all_materials(&asset_server)
        .into_iter()
        .enumerate()
    {
        let variants = [
            StandardMaterial {
                base_color_texture: base.metallic_roughness_texture.clone(),
                unlit: true,
                ..Default::default()
            },
            StandardMaterial {
                base_color_texture: base.normal_map_texture.clone(),
                unlit: true,
                ..Default::default()
            },
            StandardMaterial {
                base_color_texture: base.base_color_texture.clone(),
                unlit: true,
                ..Default::default()
            },
            base,
        ];
        for (k, mat) in variants.into_iter().enumerate() {
            spawn_sphere(mat, offset + vec3(2. * i as f32, 6.0 - 2. * k as f32, 0.));
        }
    }
}
