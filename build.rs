#[cfg(feature = "rust-gpu")]
mod rust_gpu_shaders {
    use cargo_gpu::spirv_builder::{MetadataPrintout, SpirvMetadata, SpirvTargetEnv};
    use std::fs;
    use std::path::PathBuf;

    pub fn main() -> Result<(), Box<dyn std::error::Error>> {
        let shader_crate = PathBuf::from("./rust-gpu-shaders");

        // install the toolchain and build the `rustc_codegen_spirv` codegen backend with it
        let backend = cargo_gpu::Install::from_shader_crate(shader_crate.clone()).run()?;

        // build the shader crate
        let mut builder = backend.to_spirv_builder(shader_crate, SpirvTargetEnv::Wgsl);
        builder.print_metadata = MetadataPrintout::DependencyOnly;
        builder.spirv_metadata = SpirvMetadata::Full;
        builder.multimodule = true;
        let wgsl_result = builder.build()?;
        let path_to_wgsl = wgsl_result.module.unwrap_multi();

        fs::create_dir_all("./assets/rust-gpu-shaders")?;
        for (entry_point, path) in path_to_wgsl {
            let path = path.with_extension("wgsl");
            let out_file_name = &path.file_name()[(path.file_name().rfind("::").unwrap() + 2)..];
            fs::copy(
                path,
                format!("./assets/rust-gpu-shaders/{}.wgsl", out_file_name),
            )?;
        }

        Ok(())
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "rust-gpu")]
    rust_gpu_shaders::main()?;
    Ok(())
}
