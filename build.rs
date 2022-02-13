// Currently used for moving config files into the target directory

use std::path::{Path, PathBuf};

// see https://github.com/rust-lang/cargo/issues/1759
fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = std::env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string)
        .join("target")
        .join(build_type)
        .join("configs");
    return PathBuf::from(path);
}

fn main() {
    // These values need to be synchronized with the env values in Cargo.toml
    // If anyone knows a way to export environment variables to here from Cargo.toml, it would be
    // most appreciated
    let cfg_dir = "configs";
    let cube_storage_dir = "cube_storage.json";

    let target_dir = get_output_path();

    let _ = std::fs::create_dir(&target_dir);
    let src = Path::join(&std::env::current_dir().unwrap(),
                         Path::new(cfg_dir))
        .join(Path::new(cube_storage_dir));
    let dest = Path::join(Path::new(&target_dir),
                          Path::new(cube_storage_dir));
    std::fs::copy(src, dest).unwrap();
}