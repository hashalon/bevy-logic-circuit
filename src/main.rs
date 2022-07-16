use bevy::prelude::*;


mod math;
mod voxel;
mod schematic;
mod circuit;
mod importer;


use math::*;
use voxel::*;
use schematic::*;
use circuit::*;
use importer::*;


fn main() {

    let schema = Schema::new();

    let matrix = Matrix::new(Vec3i::new(12, 12, 12), 0);

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
