/**
 * logic components to build circuitry
*/

use bevy::prelude::*;


// Data that is transmitted over wires
pub type Data = u16;

// index/color of a wire
#[derive(Component)]
pub struct PinIndex(u8);

// data on the wire on the previous tick
#[derive(Component)]
pub struct DataPrevious(Data);

// data on the wire on the next tick
#[derive(Component)]
pub struct DataNext(Data);


// inputs entering a component
#[derive(Component)]
pub struct Inputs(Vec<Entity>);

// outputs leaving a component
#[derive(Component)]
pub struct Outputs(Vec<Entity>);


// operators available
#[derive(Component)]
pub enum Operator {
    Or,
    Nor,
    And,
    Nand,
    Add,
    Mul,
}

// multiplexer
#[derive(Component)]
pub struct Mux;

// demultiplexer with output value to emit on each wire
#[derive(Component)]
pub struct Demux(Data);



// reset the state of every wire
pub fn sys_reset_wires(
    mut query: Query<(&mut DataPrevious, &mut DataNext)>
) {
    query.for_each_mut(|(mut wire_prev, mut wire_next)| {
        wire_prev.0 = wire_next.0;
        wire_next.0 = 0;
    });
}


// handle logic gates
pub fn sys_tick_gates(
    comp_query: Query<(&Operator, &Inputs, &Outputs)>,
    prev_query: Query<&DataPrevious>,
    mut next_query: Query<&mut DataNext>
) {
    for (operator, pins_in, pins_out) in comp_query.iter() {

        // find the values of input wires
        let mut values = Vec::<Data>::with_capacity(pins_in.0.len());
        for id in pins_in.0.iter() {
            if let Ok(pin) = prev_query.get(*id) {
                values.push(pin.0);
            }
        }

        // compute the output value
        let mut data: Data = 0;
        match operator {
            Operator::Or  | Operator::Nor  => for v in values.iter() {data |= v;},
            Operator::And | Operator::Nand => for v in values.iter() {data &= v;},
            Operator::Add => for v in values.iter() {data += v;},
            Operator::Mul => for v in values.iter() {data *= v;},
        }
        match operator {
            Operator::Nor | Operator::Nand => data = !data,
            _ => ()
        }

        // apply the value to all output wires
        for id in pins_out.0.iter() {
            if let Ok(mut pin) = next_query.get_mut(*id) {
                pin.0 |= data;
            }
        }
    }
}


// combine multiple input values as boolean into a single wire
pub fn sys_tick_muxes(
    comp_query: Query<(&Inputs, &Outputs), With<Mux>>,
    prev_query: Query<(&PinIndex, &DataPrevious)>,
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
pub fn sys_tick_demuxes(
    comp_query: Query<(&Inputs, &Outputs, &Demux)>,
    prev_query: Query<&DataPrevious>,
    mut next_query: Query<(&PinIndex, &mut DataNext)>
) {
    for (pins_in, pins_out, output) in comp_query.iter() {

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