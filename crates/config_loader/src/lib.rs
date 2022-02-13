mod config;

#[cfg(test)]
mod tests {
    use crate::config;

    #[test]
    fn it_works() {
        let test_val = config::load("cube_storage.json");
        assert_eq!(result, 4);
    }
}
