use std::{fs, string, io, path, option};
use serde_json::{Value, Result, from_str};

pub fn load(filename: &str) -> Result<Value> {
    let cfg_path = path::Path::new("configs").join(path::Path::new(filename));
    let cfg_path_opt = cfg_path.to_str().expect("Invalid config path");
    from_str(cfg_path_opt)
}
