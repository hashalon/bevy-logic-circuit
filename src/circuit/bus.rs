/**
 * IO Bus
 */

use bevy::prelude::*;
use crate::circuit::base::*;

// multiplexer
#[derive(Component)]
pub struct Bus;


// mux entity
#[derive(Bundle)]
pub struct BusBundle {
    pub comp: Bus,
    pub pins_in : PinsIn,
    pub pins_out: PinsOut,
}


// combine multiple input values as boolean into a single wire
pub fn sys_tick(
    comp_query: Query<(&PinsIn, &PinsOut), With<Bus>>,
    prev_query: Query<(&PinChannel, &DataPrevious)>,
    mut next_query: Query<(&PinChannel, &mut DataNext)>
) {
    for (pins_in, pins_out) in comp_query.iter() {

        // prepare stdin to read from and stdout to write to

        // write input pins data to stdout
        for id in pins_in.0.iter() {
            if let Ok((_index, _pin)) = prev_query.get(*id) {
                // TODO
            }
        }

        // read stdin data and write it to output wires
        for id in pins_out.0.iter() {
            if let Ok((_index, mut _pin)) = next_query.get_mut(*id) {
                // TODO
            }
        }
    }
}
