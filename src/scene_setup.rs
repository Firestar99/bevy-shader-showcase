use crate::camera_controller::CameraController;
use bevy::asset::AssetServer;
use bevy::core_pipeline::Skybox;
use bevy::math::Vec3;
use bevy::pbr::DirectionalLight;
use bevy::prelude::{
    default, Camera3d, Commands, Component, EnvironmentMapLight, KeyCode, PerspectiveProjection,
    Projection, Query, Res, Time, Transform, With,
};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(1.0, 1.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        Rotate,
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: 70.,
            ..Default::default()
        }),
        CameraController {
            key_down: KeyCode::ShiftLeft,
            key_up: KeyCode::Space,
            key_run: KeyCode::ControlLeft,
            scroll_factor: 0.2,
            ..CameraController::default()
        },
        Skybox {
            brightness: 2000.0,
            image: asset_server.load("textures/hdr/pisa_specular_rgb9e5_zstd.ktx2"),
            ..default()
        },
        EnvironmentMapLight {
            diffuse_map: asset_server.load("textures/hdr/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("textures/hdr/pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 900.0,
            ..default()
        },
        Transform::from_translation(Vec3::new(-2.0, 2.5, 5.0) / 3.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[derive(Component)]
pub struct Rotate;

pub fn rotate_things(mut q: Query<&mut Transform, With<Rotate>>, time: Res<Time>) {
    for mut t in &mut q {
        t.rotate_y(time.delta_secs());
    }
}
