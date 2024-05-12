use std::fmt::Display;
use std::process::{Command, Stdio};
use serde::Deserialize;

#[derive(Deserialize)]
struct CargoMessage
{
    reason: String,
}

#[derive(Deserialize)]
struct BuildScriptMessage
{
    package_id: String,
    env: Vec<Vec<String>>,
}

#[derive(Debug, Copy, Clone)]
enum Error
{
    BuildFailed,
    BuildOutputNotFound,
}

impl Display for Error
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error
{
    
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let path_to_shader = "../shader";
    let profile = "release";
    let cargo_exe = format!("{}/bin/cargo", std::env::var("CARGO_HOME").unwrap());
    let mut cargo = Command::new(cargo_exe);
    cargo.args([
        "build",
        "--lib",
        "--message-format=json-render-diagnostics",
        "--profile",
        profile,
    ]);

    // Don't use the same version of rust toolchain
    cargo.env_remove("CARGO");
    cargo.env_remove("RUSTUP_TOOLCHAIN");
    
    let build = cargo
        .stderr(Stdio::inherit())
        .current_dir(path_to_shader)
        .output()
        .expect("failed to execute cargo build");

    if !build.status.success()
    {
        return Err(Error::BuildFailed.into());
    }

    let stdout = String::from_utf8(build.stdout).unwrap();
    let mut build_script_messages = stdout.lines()
        .filter_map(|line|
        {
            let cargo_message = serde_json::from_str::<CargoMessage>(line).ok()?;
            if cargo_message.reason != "build-script-executed"
            {
                return None;
            }

            let build_script_message = serde_json::from_str::<BuildScriptMessage>(line).ok()?;
            if !build_script_message.package_id.starts_with("fractal_renderer_shader ")
            {
                return None;
            }

            Some(build_script_message)
        });

    let build_script_message = match (build_script_messages.next(), build_script_messages.next())
    {
        // If single message
        (Some(m), None) => m,
        _ => return Err(Error::BuildOutputNotFound.into()),
    };

    // Forward env variables
    for env in build_script_message.env
    {
        let [key, value] = &env[..]
        else
        {
            return Err(Error::BuildOutputNotFound.into());
        };
        
        println!("cargo::rustc-env={key}={value}");
    }

    Ok(())
}
