#import bevy_pbr::{
    mesh_view_bindings::globals,
    pbr_fragment::pbr_input_from_standard_material,
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing, alpha_discard},
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {
    var pbr_input = pbr_input_from_standard_material(in, is_front);
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

    var out_color = apply_pbr_lighting(pbr_input);

    let quantization = 10.f;
    out_color = vec4<f32>(vec4<u32>(out_color * f32(quantization))) / f32(quantization);

    out_color = main_pass_post_lighting_processing(pbr_input, out_color);

    return out_color;
}
