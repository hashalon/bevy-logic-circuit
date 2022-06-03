use bevy::prelude::*;


mod schematic;
mod circuit;
//mod importer;


use schematic::*;
use circuit::*;
//use importer::*;


fn main() {
    let schema = Schema::new();
    let res = schema.verify();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CircuitPlugin)
        .run();
}
