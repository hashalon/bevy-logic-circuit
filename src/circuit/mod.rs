/**
 * Plugin for running logic circuits
 */
use bevy::prelude::*;


mod base;
mod fixed;
mod gate;
mod mux;
mod demux;
mod io_bus;
mod input;

// types to export
pub use base  ::*;
pub use io_bus::CompIOBus;
pub use fixed ::CompFixed;
pub use gate  ::Operator;
pub use mux   ::CompMux;
pub use demux ::CompDemux;
pub use input ::{InputDevice, CompInput};


// plugin for running the circuit
pub struct CircuitPlugin;


// labels to indicate the execution order of systems
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum Label {
    Tick,
    Tock,
}


impl Plugin for CircuitPlugin {
    fn build(&self, app: &mut App) {
        app

        // add singleton components as resources
        .insert_resource(InputDevice::default())

        // reset before next tick
        .add_system(       sys_tock.before(Label::Tick))
        .add_system(input::sys_tock.before(Label::Tick))

        // tick update
        .add_system(io_bus::sys_tick.label(Label::Tick))
        .add_system(gate  ::sys_tick.label(Label::Tick))
        .add_system(fixed ::sys_tick.label(Label::Tick))
        .add_system(input ::sys_tick.label(Label::Tick))
        .add_system(mux   ::sys_tick.label(Label::Tick))
        .add_system(demux ::sys_tick.label(Label::Tick));
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