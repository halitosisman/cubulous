mod config {
    use std::{fs, string, io};
    use serde_json::{Value, Result, from_str};

    pub fn load(filename: &str) -> Result<Value> {
        let file_str: String = fs::read_to_string(filename)?;
        from_str(file_str.as_str())?
    }
}