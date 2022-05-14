use bevy::prelude::*;

mod base;
mod gate;
mod mux;

pub use base::*;
pub use gate::Operator;
pub use mux::{Mux, Demux};


// plugin for running the circuit
pub struct CircuitPlugin;


impl Plugin for CircuitPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(sys_reset)
        .add_system(gate::sys_tick     .after(sys_reset))
        .add_system(mux::sys_tick_mux  .after(sys_reset))
        .add_system(mux::sys_tick_demux.after(sys_reset));

    }
}
