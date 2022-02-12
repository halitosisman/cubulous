use bevy::prelude::*;
mod placeholders_plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(placeholders_plugin::HelloPlugin)
        .run();
}
