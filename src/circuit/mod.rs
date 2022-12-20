/**
 * Plugin for running logic circuits
 */
use bevy::prelude::*;


mod base;
mod wire;
mod fixed;
mod gate;
mod mux;
mod bus;
mod input;

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
pub use fixed::{Fixed, FixedBundle};
pub use gate::{Operator, GateBundle};
pub use mux::{Mux, Demux, MuxBundle, DemuxBundle};
pub use bus::{Bus, BusBundle};
pub use input::{DataBuffer, Connector, InputBundle};


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

        // add singleton components as resources
        .insert_resource(DataBuffer::default())

        // reset before next tick
        .add_system(wire ::sys_reset.label(Label::Reset))
        .add_system(input::sys_reset.label(Label::Reset))

        // tick update
        .add_system(bus  ::sys_tick.after(Label::Reset))
        .add_system(gate ::sys_tick.after(Label::Reset))
        .add_system(fixed::sys_tick.after(Label::Reset))
        .add_system(input::sys_tick.after(Label::Reset))
        .add_system(mux  ::sys_tick_mux  .after(Label::Reset))
        .add_system(mux  ::sys_tick_demux.after(Label::Reset));
    }
}
