use bevy::prelude::*;


mod math;
mod circuit;
mod builder;


use math::*;
use circuit::*;
use builder::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CircuitPlugin)
        .run();
}
