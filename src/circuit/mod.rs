/**
 * Plugin for running logic circuits
 */
use bevy::prelude::*;

mod base;
mod demux;
mod fixed;
mod gate;
mod input;
mod io_bus;
mod mux;

// types to export
pub use base::*;
pub use demux::CompDemux;
pub use fixed::CompFixed;
pub use gate::Operator;
pub use input::{CompInput, InputDevice};
pub use io_bus::CompIOBus;
pub use mux::CompMux;

// plugin for running the circuit
pub struct CircuitPlugin;

impl Plugin for CircuitPlugin {
    fn build(&self, app: &mut App) {
        app
            // add singleton components as resources
            .insert_resource(InputDevice::default())
            // reset before next tick
            .add_systems(PreUpdate, (sys_tock, input::sys_tock))
            // tick update
            .add_systems(
                Update,
                (
                    io_bus::sys_tick,
                    gate::sys_tick,
                    fixed::sys_tick,
                    input::sys_tick,
                    mux::sys_tick,
                    demux::sys_tick,
                ),
            );
    }
}

/* Wire Entity: PinChannel, DataPrev, DataNext */
// reset the state of every wire
fn sys_tock(mut query: Query<(&mut DataPrev, &mut DataNext)>) {
    query.for_each_mut(|(mut wire_prev, mut wire_next)| {
        wire_prev.0 = wire_next.0;
        wire_next.0 = 0;
    });
}
