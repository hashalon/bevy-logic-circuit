/**
 * multiplexer and demultiplexer
 */

use bevy::prelude::*;
use crate::circuit::base::*;

// multiplexer
#[derive(Component)]
pub struct Mux;

// demultiplexer with output value to emit on each wire
#[derive(Component)]
pub struct Demux(Data);


// mux entity
#[derive(Bundle)]
pub struct MuxBundle {
    pub comp: Mux,
    pub pins_in : PinsIn,
    pub pins_out: PinsOut,
}

// mux entity
#[derive(Bundle)]
pub struct DemuxBundle {
    pub comp: Demux,
    pub pins_in : PinsIn,
    pub pins_out: PinsOut,
}


// combine multiple input values as boolean into a single wire
pub fn sys_tick_mux(
    comp_query: Query<(&PinsIn, &PinsOut), With<Mux>>,
    prev_query: Query<(&PinChannel, &DataPrevious)>,
    mut next_query: Query<&mut DataNext>
) {
    for (pins_in, pins_out) in comp_query.iter() {

        // find the values of input wires
        let mut data: Data = 0;
        for id in pins_in.0.iter() {
            if let Ok((index, pin)) = prev_query.get(*id) {
                data |= if pin.0 != 0 {1} else {0} << index.0;
            }
        }

        // apply the value to all output wires
        for id in pins_out.0.iter() {
            if let Ok(mut pin) = next_query.get_mut(*id) {
                pin.0 |= data;
            }
        }
    }
}

// split an input value into multiple boolean output
pub fn sys_tick_demux(
    comp_query: Query<(&Demux, &PinsIn, &PinsOut)>,
    prev_query: Query<&DataPrevious>,
    mut next_query: Query<(&PinChannel, &mut DataNext)>
) {
    for (output, pins_in, pins_out) in comp_query.iter() {

        // find the values of input wires
        let mut data: Data = 0;
        for id in pins_in.0.iter() {
            if let Ok(pin) = prev_query.get(*id) {
                data |= pin.0;
            }
        }

        // apply the value to all output wires
        for id in pins_out.0.iter() {
            if let Ok((index, mut pin)) = next_query.get_mut(*id) {
                if ((data >> index.0) & 1) != 1 {
                    pin.0 |= output.0;
                }
            }
        }
    }
}
