/**
 * Plugin for running logic circuits
 */
use bevy::prelude::*;


mod base;
mod wire;
mod constant;
mod gate;
mod mux;
mod keyboard;

// types to export
pub use base::{
    CHANNELS,
    DATA_SIZE,
    Data,
    DataPrevious,
    DataNext,
    PinIndex,
    PinsIn,
    PinsOut,
    Operator,
    CompType,
};
pub use wire::WireBundle;
pub use constant::{Constant, ConstBundle};
pub use gate::GateBundle;
pub use mux::{
    Mux,
    Demux,
    MuxBundle,
    DemuxBundle,
};
pub use keyboard::{
    KeyboardDevice,
    KeyboardConnector,
    KeyboardBundle,
};


// plugin for running the circuit
pub struct CircuitPlugin;


// labels to indicate the execution order of systems
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum Label {
    Reset,
}


// build a wire entity
pub fn start_build_wires(mut commands: Commands) {
    for wire in 0..1 {
        let entity = commands.spawn()
            .insert_bundle(WireBundle {
                index: PinIndex(0),
                prev: DataPrevious(0),
                next: DataNext(0),
            });
    }
}


// build a constant entity
pub fn start_build_components(mut commands: Commands) {
    for comp in 0..1 {
        let entity = commands.spawn()
            .insert_bundle(WireBundle {
                index: PinIndex(0),
                prev: DataPrevious(0),
                next: DataNext(0),
            });
    }
}


impl Plugin for CircuitPlugin {
    fn build(&self, app: &mut App) {
        app

        // reset before next tick
        .add_system(wire::sys_reset    .label(Label::Reset))
        .add_system(keyboard::sys_reset.label(Label::Reset))

        // tick update
        .add_system(constant::sys_tick .after(Label::Reset))
        .add_system(gate::sys_tick     .after(Label::Reset))
        .add_system(mux::sys_tick_mux  .after(Label::Reset))
        .add_system(mux::sys_tick_demux.after(Label::Reset))
        .add_system(keyboard::sys_tick .after(Label::Reset));
    }
}
