use bevy::prelude::*;
mod loader;
mod bespoke_tree;

#[derive(Default)]
struct Config {
    cubes_in_leaf: u32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
