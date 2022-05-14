use bevy::prelude::*;


mod circuit;

use circuit::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CircuitPlugin)
        .run();
}
