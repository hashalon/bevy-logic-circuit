/**
 * Plugin for running logic circuits
 */
use bevy::prelude::*;


mod base;
mod wire;
mod constant;
mod gate;
mod mux;
mod bus;
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
    BundleModel,
};
pub use wire::WireBundle;
pub use constant::{Constant, BundleConst};
pub use gate::{Operator, BundleGate};
pub use mux::{Mux, Demux, BundleMux, BundleDemux};
pub use bus::{Bus, BundleBus};
pub use keyboard::{
    KeyboardDevice,
    KeyboardConnector,
    BundleKeyboard,
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
        .add_system(wire::    sys_reset.label(Label::Reset))
        .add_system(keyboard::sys_reset.label(Label::Reset))

        // tick update
        .add_system(constant:: sys_tick.after(Label::Reset))
        .add_system(gate::     sys_tick.after(Label::Reset))
        .add_system(mux::  mux_sys_tick.after(Label::Reset))
        .add_system(mux::demux_sys_tick.after(Label::Reset))
        .add_system(bus::      sys_tick.after(Label::Reset))
        .add_system(keyboard:: sys_tick.after(Label::Reset));
    }
}
