#import bevy_pbr::{
    mesh_view_bindings::globals,
    pbr_fragment::pbr_input_from_standard_material,
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing, alpha_discard},
}

@fragment
fn fragment(
    in_original: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {
    let speed = 0.5;
    let t_1 = sin(globals.time * speed) * 0.5 + 0.5;
    let t_2 = cos(globals.time * speed);
    var in = in_original;
    in.uv += vec2(t_1, t_2);

    var pbr_input = pbr_input_from_standard_material(in, is_front);
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

    var out_color = apply_pbr_lighting(pbr_input);

    out_color = main_pass_post_lighting_processing(pbr_input, out_color);

    return out_color;
}
