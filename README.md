# Bevy shader showcase

This repo contains a variety of custom wgsl shaders for [bevy](https://bevy.org/). It was made as an introduction to shader programming.

* [wgsl shader folder](./assets/shaders/)
* [bevy material loading code folder](./src/custom_shaders/)

### Samples
* a: basic color output [wgsl](./assets/shaders/a_basic_color.wgsl) [bevy](./src/custom_shaders/a_basic_color.rs)
* b: display texture coordinates [wgsl](./assets/shaders/b_tex_coords.wgsl) [bevy](./src/custom_shaders/b_tex_coords.rs)
* c: a circle made with shaders [wgsl](./assets/shaders/c_circle.wgsl) [bevy](./src/custom_shaders/c_circle.rs)
* d: simple animated effect [wgsl](./assets/shaders/d_animated.wgsl) [bevy](./src/custom_shaders/d_animated.rs)
* e: sampling textures [wgsl](./assets/shaders/e_texture.wgsl) [bevy](./src/custom_shaders/e_texture.rs)
* f: discard to cut out textures [wgsl](./assets/shaders/f_discard.wgsl) [bevy](./src/custom_shaders/f_discard.rs)
* g: color blending and it's difficulties [wgsl](./assets/shaders/g_blending.wgsl) [bevy](./src/custom_shaders/g_blending.rs)
* h: what are normals? [wgsl](./assets/shaders/h_normals.wgsl) [bevy](./src/custom_shaders/h_normals.rs)
* i: basic diffuse lighting [wgsl](./assets/shaders/i_diffuse_lighting.wgsl) [bevy](./src/custom_shaders/i_diffuse_lighting.rs)
* j: adding some ambient light [wgsl](./assets/shaders/j_ambient_lighting.wgsl) [bevy](./src/custom_shaders/j_ambient_lighting.rs)
* k: add specular to get phong lighting [wgsl](./assets/shaders/k_phong_lighting.wgsl) [bevy](./src/custom_shaders/k_phong_lighting.rs)
* l: bevy PBR lighting with just basic color [bevy](./src/custom_shaders/l_pbr_base_color.rs)
* m: all the PBR material properties [bevy](./src/custom_shaders/m_pbr_materials.rs)
* n: animated PBR textures [wgsl](./assets/shaders/n_pbr_animated.wgsl) [bevy](src/custom_shaders/n_pbr_animated.rs)
* o: cell shading with PBR [wgsl](./assets/shaders/o_pbr_cell_shaded.wgsl) [bevy](./src/custom_shaders/o_pbr_cell_shaded.rs)
