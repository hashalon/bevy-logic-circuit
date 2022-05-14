use bevy::prelude::*;


mod builder;
mod circuit;

use builder::*;
use circuit::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CircuitPlugin)
        .run();
}
