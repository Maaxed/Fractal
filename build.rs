use std::path::Path;

use spirv_builder::{SpirvBuilder, Capability, CompileResult, SpirvBuilderError};

const TARGET: &str = "spirv-unknown-spv1.5";

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    build_shader("./shader/vertex32", false)?;
    build_shader("./shader/vertex64", true)?;
    build_shader("./shader/fragment", false)?;
    build_shader("./shader/compute32", false)?;
    build_shader("./shader/compute64", true)?;

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

    builder.build()
}