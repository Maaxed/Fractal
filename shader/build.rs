use std::path::Path;

use spirv_builder::{SpirvBuilder, Capability, CompileResult, SpirvBuilderError};

const TARGET: &str = "spirv-unknown-spv1.5";

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    build_shader("vertex32", false)?;
    build_shader("vertex64", true)?;
    build_shader("fragment", false)?;

    //build_shader("compute32", false)?;
    //build_shader("compute64", true)?;
    
    build_shader("computation32", false)?;
    build_shader("computation64", true)?;

    Ok(())
}

fn build_shader(path_to_crate: impl AsRef<Path>, uses_float64: bool) -> Result<CompileResult, SpirvBuilderError>
{
    let mut builder = SpirvBuilder::new(path_to_crate, TARGET)
        .deny_warnings(true);

    if uses_float64
    {
        builder = builder.capability(Capability::Float64);
    }

    match std::env::var("PROFILE").as_deref()
    {
        Ok("release") => builder = builder.release(true),
        Ok("debug" | "dev") => builder = builder.release(true),
        _ => {},
    }

    builder.build()
}