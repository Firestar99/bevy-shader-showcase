#import bevy_pbr::{
    forward_io::{VertexOutput},
    mesh_view_bindings::lights,
}

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var material_color_texture: texture_2d<f32>;
@group(2) @binding(2) var material_color_sampler: sampler;

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let light_color = vec3f(1.0, 1.0, 1.0);
//    let light_color = lights.directional_lights[0].color.xyz;

    let light_dir = lights.directional_lights[0].direction_to_light;
    let normal = normalize(in.world_normal);
    let diffuse_factor: f32 = max(dot(light_dir, normal), 0.);
    let diffuse_light = diffuse_factor * light_color;

    let ambient_light = vec3f(0.1);
//    let ambient_light = lights.ambient_light;

    let material_color = textureSample(material_color_texture, material_color_sampler, in.uv) * material_color;
    let color = (diffuse_light + ambient_light) * material_color.xyz;
    return vec4f(color, 1.0);
}
