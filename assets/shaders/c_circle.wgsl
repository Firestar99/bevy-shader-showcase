#import bevy_pbr::{
    forward_io::{VertexOutput},
}

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
//    let uv = in.uv;
//    let uv = in.uv - 0.5;
//    return vec4f(uv, 0.0, 1.0);

//    let distance = length(in.uv - 0.5);
    let distance = 0.5 - length(in.uv - 0.5);
    return vec4f(vec3f(distance), 1.0);
}
