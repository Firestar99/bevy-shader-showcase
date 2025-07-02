use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::prelude::*;

pub fn learn_opengl_material(asset_server: &Res<AssetServer>, path: &str) -> StandardMaterial {
    let settings = |s: &mut _| {
        *s = ImageLoaderSettings {
            sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                // rewriting mode to repeat image,
                address_mode_u: ImageAddressMode::MirrorRepeat,
                address_mode_v: ImageAddressMode::MirrorRepeat,
                ..default()
            }),
            ..default()
        }
    };

    StandardMaterial {
        base_color_texture: Some(
            asset_server.load_with_settings(format!("{path}/albedo.png"), settings),
        ),
        normal_map_texture: Some(
            asset_server.load_with_settings(format!("{path}/normal.png"), settings),
        ),
        metallic_roughness_texture: Some(
            asset_server.load_with_settings(format!("{path}/metallic_roughness.png"), settings),
        ),
        metallic: 1.0,
        perceptual_roughness: 1.0,
        ..Default::default()
    }
}

pub fn gold(asset_server: &Res<AssetServer>) -> StandardMaterial {
    learn_opengl_material(asset_server, "textures/pbr/gold")
}

pub fn grass(asset_server: &Res<AssetServer>) -> StandardMaterial {
    learn_opengl_material(asset_server, "textures/pbr/grass")
}

pub fn plastic(asset_server: &Res<AssetServer>) -> StandardMaterial {
    learn_opengl_material(asset_server, "textures/pbr/plastic")
}

pub fn rusted_iron(asset_server: &Res<AssetServer>) -> StandardMaterial {
    learn_opengl_material(asset_server, "textures/pbr/rusted_iron")
}

pub fn wall(asset_server: &Res<AssetServer>) -> StandardMaterial {
    learn_opengl_material(asset_server, "textures/pbr/wall")
}

pub fn all_materials(asset_server: &Res<AssetServer>) -> Vec<StandardMaterial> {
    Vec::from([
        gold(asset_server),
        grass(asset_server),
        plastic(asset_server),
        rusted_iron(asset_server),
        wall(asset_server),
    ])
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mut spawn_sphere = |mat: StandardMaterial, offset: Vec3| {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::default())),
            MeshMaterial3d(materials.add(mat)),
            Transform::from_translation(offset),
        ));
    };

    let offset = vec3(24.0, 0.5, 0.0);
    for (i, base) in all_materials(&asset_server).into_iter().enumerate() {
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
