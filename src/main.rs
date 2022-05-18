use bevy::prelude::*;


mod math;
mod circuit;
mod importer;


use math::*;
use circuit::*;
use importer::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CircuitPlugin)
        .run();
}
