use glam::{Vec2, Vec3, Vec4};
use spirv_std::spirv;

#[spirv(fragment)]
pub fn b_tex_coords(_world_position: Vec4, _world_normal: Vec3, uv: Vec2, output: &mut Vec4) {
    *output = Vec4::from((uv, 0.0, 1.0));
}
