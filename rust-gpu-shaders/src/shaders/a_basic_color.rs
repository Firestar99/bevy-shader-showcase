use glam::{Vec2, Vec3, Vec4};
use spirv_std::spirv;

#[spirv(fragment)]
pub fn a_basic_color(_world_position: Vec4, _world_normal: Vec3, _uv: Vec2, output: &mut Vec4) {
    *output = Vec4::new(1.0, 0.5, 0.0, 1.0);
}
