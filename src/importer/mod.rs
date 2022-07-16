

mod base;
mod xraw;

use base::*;
use xraw::*;

/*

// build a wire entity
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
