use glam::{Vec2, Vec3, Vec4};
use spirv_std::spirv;

#[spirv(fragment)]
pub fn c_circle(_world_position: Vec4, _world_normal: Vec3, uv: Vec2, output: &mut Vec4) {
    //    let uv = in.uv;
    //    let uv = in.uv - 0.5;
    *output = Vec4::from((uv, 0.0, 1.0));

    //    let distance = (in.uv - 0.5).length();
    let distance = 0.5 - (uv - 0.5).length();
    *output = Vec4::from((Vec3::splat(distance), 1.0));
}
