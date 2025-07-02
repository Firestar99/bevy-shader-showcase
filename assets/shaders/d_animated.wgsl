#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::{VertexOutput},
}

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let distance = 0.707 - length(in.uv - 0.5);

    let speed = 2.0;
    let t_1 = sin(globals.time * speed) * 0.5 + 0.5;
    let t_2 = cos(globals.time * speed);

    let red = vec3f(1., 0., 0.);
    let green = vec3f(0., 1., 0.);
    let blue = vec3f(0., 0., 1.);
    let white = vec3<f32>(1., 1., 1.);
    let mixed = mix(mix(red, blue, t_1), mix(green, white, t_2), distance);
    return vec4f(mixed, 1.0);
}
