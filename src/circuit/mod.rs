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
    NB_CHANNELS,
    DATA_SIZE,
    Channel,
    Data,
    DataPrevious,
    DataNext,
    PinChannel,
    PinsIn,
    PinsOut,
};
pub use wire::WireBundle;
pub use constant::{Constant, ConstBundle};
pub use gate::{Operator, GateBundle};
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
