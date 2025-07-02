use crate::camera_controller::CameraControllerPlugin;
use bevy::{pbr::ExtendedMaterial, prelude::*};
use custom_shaders::*;

mod custom_shaders;

mod base_plane;
mod camera_controller;
pub mod double_sided;
mod scene_setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraControllerPlugin)
        .add_systems(Startup, scene_setup::setup)
        .add_systems(Update, scene_setup::rotate_things)
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
        .add_systems(Startup, l_pbr_base_color::setup)
        .add_systems(Startup, m_pbr_materials::setup)
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, n_pbr_shaders::CellShadedMaterialExtension>,
        >::default())
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, n_pbr_shaders::AnimatedMaterialExtension>,
        >::default())
        .add_systems(Startup, n_pbr_shaders::setup)
        .run();
}
