use crate::camera_controller::{CameraController, CameraControllerPlugin};
use bevy::{pbr::ExtendedMaterial, prelude::*};
use custom_shaders::*;

mod custom_shaders;

mod base_plane;
mod camera_controller;
pub mod double_sided;
mod extended_material;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraControllerPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_things)
        .add_systems(Startup, base_plane::setup)
        .add_plugins(MaterialPlugin::<a_basic_color::CustomMaterial>::default())
        .add_systems(Startup, a_basic_color::setup)
        .add_plugins(MaterialPlugin::<b_tex_coords::CustomMaterial>::default())
        .add_systems(Startup, b_tex_coords::setup)
        .add_plugins(MaterialPlugin::<c_circle::CustomMaterial>::default())
        .add_systems(Startup, c_circle::setup)
        .add_plugins(MaterialPlugin::<d_animated::CustomMaterial>::default())
        .add_systems(Startup, d_animated::setup)
        .add_plugins(MaterialPlugin::<e_texture::CustomMaterial>::default())
        .add_systems(Startup, e_texture::setup)
        .add_plugins(MaterialPlugin::<f_discard::CustomMaterial>::default())
        .add_systems(Startup, f_discard::setup)
        .add_plugins(MaterialPlugin::<g_blending::CustomMaterial>::default())
        .add_systems(Startup, g_blending::setup)
        .add_plugins(MaterialPlugin::<h_normals::CustomMaterial>::default())
        .add_systems(Startup, h_normals::setup)
        .add_plugins(MaterialPlugin::<i_diffuse_lighting::CustomMaterial>::default())
        .add_systems(Startup, i_diffuse_lighting::setup)
        .add_plugins(MaterialPlugin::<j_ambient_lighting::CustomMaterial>::default())
        .add_systems(Startup, j_ambient_lighting::setup)
        .add_plugins(MaterialPlugin::<k_phong_lighting::CustomMaterial>::default())
        .add_systems(Startup, k_phong_lighting::setup)
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, extended_material::MyExtension>,
        >::default())
        .add_systems(Startup, extended_material::setup)
        .run();
}

fn setup(mut commands: Commands) {
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
            fov: 90.,
            ..Default::default()
        }),
        CameraController {
            key_down: KeyCode::ShiftLeft,
            key_up: KeyCode::Space,
            key_run: KeyCode::ControlLeft,
            scroll_factor: 0.2,
            ..CameraController::default()
        },
        Transform::from_translation(Vec3::new(-2.0, 2.5, 5.0) / 3.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[derive(Component)]
struct Rotate;

fn rotate_things(mut q: Query<&mut Transform, With<Rotate>>, time: Res<Time>) {
    for mut t in &mut q {
        t.rotate_y(time.delta_secs());
    }
}
