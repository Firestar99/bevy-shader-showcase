use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let learn_opengl_material = |path: &str| StandardMaterial {
        base_color_texture: Some(asset_server.load(format!("{path}/albedo.png"))),
        normal_map_texture: Some(asset_server.load(format!("{path}/normal.png"))),
        metallic_roughness_texture: Some(
            asset_server.load(format!("{path}/metallic_roughness.png")),
        ),
        metallic: 1.0,
        perceptual_roughness: 1.0,
        ..Default::default()
    };
    let all_materials = [
        learn_opengl_material("textures/pbr/gold"),
        learn_opengl_material("textures/pbr/grass"),
        learn_opengl_material("textures/pbr/plastic"),
        learn_opengl_material("textures/pbr/rusted_iron"),
        learn_opengl_material("textures/pbr/wall"),
    ];

    let mut spawn_sphere = |mat: StandardMaterial, offset: Vec3| {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::default())),
            MeshMaterial3d(materials.add(mat)),
            Transform::from_translation(offset),
        ));
    };

    let offset = vec3(24.0, 0.5, 0.0);
    for (i, base) in all_materials.into_iter().enumerate() {
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
