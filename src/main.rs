use bevy::prelude::*;


mod schematic;
mod circuit;
//mod importer;


use schematic::*;
use circuit::*;
//use importer::*;


fn main() {
    let schema = Schema::new();

    match schema.verify() {
        Ok(_) => {

        },
        Err(errors) => {
            // there was error, it is not possible to build the circuit
            for error in errors.iter() {
                println!("{}", error.message());
            }
        }
    }

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CircuitPlugin)
        .run();
}
