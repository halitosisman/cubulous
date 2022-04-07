// Init time constants
use bevy::prelude::*;

#[derive(Default)]
struct CubeConfig {
    storage_root: String,

}

impl FromWorld for CubeConfig {
    fn from_world(world: &mut World) -> Self {
        // Get the top level game filesystem root
        let root = world.get_resource()
        CubeConfig {
            .storage_root =
        }
    }
}