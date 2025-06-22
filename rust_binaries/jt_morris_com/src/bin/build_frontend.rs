// param($out_dir = "$PsScriptRoot/dest")
// $rust_target = "wasm32-unknown-unknown"
// $crate_name = "login_page"
// rustup update
// rustup target add $rust_target
// cargo install wasm-pack
// cargo build -p $crate_name --release --target wasm32-unknown-unknown
// $wasm_file = "$(Get-Location)\target\$rust_target\release\$crate_name.wasm"
// wasm-bindgen  --target web --out-dir $out_dir $wasm_file

use core::str;
use std::{io::Read, path::PathBuf, process::Command, str::FromStr};

use serde::Deserialize;

// const CONFIG_TOML_STR: &str = include_str!("../Cargo.toml");
// const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
// const DEFAULT_OUT_DIR: &str = "dist";
const WASM_BINDGEN_TARGET: &str = "web";
const RUST_TARGET: &str = "wasm32-unknown-unknown";
#[derive(Deserialize, Debug)]
pub struct PackageMetadata {
    name: String,
    // version:String,
    // id:String,
    // edition:String,
    // license:Option<String>,
    // description:Option<String>,
    // #[serde(flatten)]
    // other: HashMap<String,serde_json::Value>
}
#[derive(Deserialize, Debug)]
pub struct CargoTomlFile {
    package: PackageMetadata,
}

#[derive(Deserialize, Debug)]
pub struct CargoMetadata {
    packages: Vec<PackageMetadata>,
    // workspace_members:Vec<String>,
    // resolve: HashMap<String,serde_json::Value>,
    target_directory: std::path::PathBuf,
    workspace_root: std::path::PathBuf,
    // #[serde(flatten)]
    // other: HashMap<String,serde_json::Value>
    // workspace_root: String
}

/// attempts to read a Rocket.toml file at the workspace root to determine where the public files are locate
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
fn get_rocket_public_files_root(
    workspace_root: &PathBuf,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // first try to get it from the environment variables
    let mut output = String::new();
    let mut f = std::fs::File::open(workspace_root.join("Rocket.toml"))?;
    let _ = f.read_to_string(&mut output)?;
    let toml_file: toml::Table =
        toml::from_str(&output).map_err(|e| Box::<dyn std::error::Error>::from(e))?;
    let profile_str = std::env::var("ROCKET_PROFILE").unwrap_or_else(|_| "default".to_string());
    let table = toml_file.get(&profile_str).ok_or_else(|| {
        Box::<dyn std::error::Error>::from("expected profile for Rocket.toml but did not find it")
    })?;
    Ok(table
        .get("public_files_root")
        .map(|v| v.as_str())
        .flatten()
        .map(|s| PathBuf::from_str(s).unwrap())
        .unwrap_or(workspace_root.join("public")))
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv().ok();
    // compile_error!("TODO: implement reading defaults/settings from the crate's Cargo.toml");
    // get a list of installed cargo binaries
    let cargo_install = Command::new("cargo")
        .args(&["install", "--list"])
        .output()?;

    let cargo_install_output = str::from_utf8(cargo_install.stdout.as_slice())?;

    // install the reqiured dependencies if not provided in the output
    let required_programs: [&str; 2] = ["wasm-bindgen-cli", "wasm-opt"];

    for program in required_programs.iter() {
        if !cargo_install_output.contains(program) {
            let _ = Command::new("cargo")
                .arg("install")
                .args(&required_programs)
                .output()?;
            break;
        }
    }

    // run cargo metadata
    let metadata_command = Command::new("cargo")
        .args(&["metadata", "--format-version=1"])
        .output()?;
    if !metadata_command.status.success() {
        // eprintln!("{}",)
        return Err(String::from_utf8_lossy(&metadata_command.stderr).into());
    }
    let metadata_string = String::from_utf8(metadata_command.stdout)?;
    let cargo_metadata: CargoMetadata = serde_json::from_str(&metadata_string)?;

    let cargo_toml: CargoTomlFile = {
        let mut output = String::new();
        std::fs::File::open(cargo_metadata.workspace_root.join("Cargo.toml"))?
            .read_to_string(&mut output)?;
        toml::from_str(&output)?
    };

    // attempt to read the webapps config file

    // // determine the output directory for wasm-bindgen-cli
    let out_dir = match std::env::args().nth(1) {
        Some(out_str) => std::path::PathBuf::from_str(&out_str)?,
        None => get_rocket_public_files_root(&cargo_metadata.workspace_root)
            .unwrap_or_else(|_| cargo_metadata.workspace_root.join("public")),
    }
    .join(&cargo_toml.package.name);
    // run cargo build

    let _ = Command::new("cargo")
        .args(&[
            "build",
            "-p",
            &cargo_toml.package.name,
            "--target",
            RUST_TARGET,
            "--release",
        ])
        .status()?;

    const RELEASE_MODE: &str = "release";
    let wasm_target = WASM_BINDGEN_TARGET;
    let wasm_dir = cargo_metadata
        .target_directory
        .join(RUST_TARGET)
        .join(RELEASE_MODE)
        .join(format!("{}.wasm", &cargo_toml.package.name))
        .to_string_lossy()
        .to_string();
    // run wasm_bingen
    let _ = std::process::Command::new("wasm-bindgen")
        .args(&[
            "--out-dir",
            out_dir.display().to_string().as_str(),
            "--target",
            &wasm_target,
            &wasm_dir,
        ])
        .status()?;
    // run wasm_opt
    let bindgen_wasm_path = out_dir.join(format!("{}_bg.wasm", &cargo_toml.package.name));
    wasm_opt::OptimizationOptions::new_opt_level_2().run(&bindgen_wasm_path, &bindgen_wasm_path)?;
    Ok(())
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn main() {}
