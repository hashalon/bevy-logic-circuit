/**
 * Plugin for running logic circuits
 */
use bevy::prelude::*;


mod base;
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
    Inputs,
    Outputs,
    Constant,
};
pub use gate::Operator;
pub use mux::{Mux, Demux};


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
        .add_system(base::sys_reset    .label(Label::Reset))
        .add_system(keyboard::sys_reset.label(Label::Reset))

        // tick update
        .add_system(base::sys_tick     .after(Label::Reset))
        .add_system(gate::sys_tick     .after(Label::Reset))
        .add_system(mux::sys_tick_mux  .after(Label::Reset))
        .add_system(mux::sys_tick_demux.after(Label::Reset))
        .add_system(keyboard::sys_tick .after(Label::Reset));
    }
}
