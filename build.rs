// Currently used for moving config files into the target directory

use std::path::{Path, PathBuf};

fn get_target_dir() -> PathBuf {
    let manifest_dir_string = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = std::env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string)
        .join("target")
        .join(build_type);

    return path;
}

// see https://github.com/rust-lang/cargo/issues/1759
fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let path = get_target_dir().join("configs");

    return path;
}

fn get_worlds_path() -> PathBuf {
    get_target_dir().join("worlds")
}

fn main() {
    // These values need to be synchronized with the env values in Cargo.toml
    // If anyone knows a way to export environment variables to here from Cargo.toml, it would be
    // most appreciated

    // Create the cube config directory and copy the default config to it
    let cfg_dir = "configs";
    let cube_storage_dir = "cube_storage.json";

    let config_dir = get_output_path();

    std::fs::create_dir(&config_dir).unwrap();
    let src = Path::join(&std::env::current_dir().unwrap(),
                         Path::new(cfg_dir))
        .join(Path::new(cube_storage_dir));
    let dest = Path::join(Path::new(&config_dir),
                          Path::new(cube_storage_dir));
    std::fs::copy(src, dest).unwrap();

    // Create the worlds directory for testing
    let worlds_path = get_worlds_path();
    std::fs::create_dir(&worlds_path);
    std::fs::create_dir(&(worlds_path.join("test_world")));
}