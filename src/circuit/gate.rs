/**
 * logic components to build circuitry
*/

use bevy::prelude::*;
use crate::circuit::base::*;


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


// handle logic gates
pub fn sys_tick(
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
