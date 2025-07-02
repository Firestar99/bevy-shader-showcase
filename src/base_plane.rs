use bevy::asset::Assets;
use bevy::color::Color;
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{Commands, Mesh, Mesh3d, Meshable, Plane3d, ResMut, Transform};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    let length = 50.0;
    let border_radius = 5.0;

    // base plane
    commands.spawn((
        Mesh3d(
            meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(length + border_radius * 2., border_radius * 2.),
            ),
        ),
        MeshMaterial3d(standard_materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            unlit: true,
            ..Default::default()
        })),
        Transform::from_xyz(length / 2., -2.0, 0.0),
    ));
}
