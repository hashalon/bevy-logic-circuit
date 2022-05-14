use bevy::prelude::*;
use crate::circuit::*;

mod voxel;
mod loader_xraw;

pub use voxel::{
    Vec3i,
    Type,
    Model,
};
pub use loader_xraw::load_xraw;



/* build a wire entity
pub fn start_load_circuit(mut commands: Commands, server: Res<AssetServer>) {
    let handle: Handle<> = server.load();

    commands.insert_resource();

    let entity = commands.spawn()
        .insert_bundle(WireBundle {
            index: PinIndex(0),
            prev: DataPrevious(0),
            next: DataNext(0),
        });
}
*/