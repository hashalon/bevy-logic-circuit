/**
 * logic components to build circuitry
 */

use bevy::prelude::*;
use crate::circuit::base::*;
use serde::{Deserialize, Serialize};
use std::cmp::{min, max};


// operators available
#[derive(Clone, Copy, Component, Deserialize, Serialize)]
pub enum Operator {
    Or,
    And,
    Nor,
    Nand,
    Add,
    Mul,
    Min,
    Max,
}


// constant entity
#[derive(Bundle)]
pub struct GateBundle {
    pub comp    : Operator,
    pub pins_in : PinsIn,
    pub pins_out: PinsOut,
}


// handle logic gates
pub fn sys_tick(
    comp_query: Query<(&Operator, &PinsIn, &PinsOut)>,
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
            Operator::Or  => values.iter().for_each(|v| data |= *v),
            Operator::And => values.iter().for_each(|v| data &= *v),
            Operator::Nor => {
                values.iter().for_each(|v| data |= *v);
                data = !data;
            },
            Operator::Nand => {
                values.iter().for_each(|v| data &= *v);
                data = !data;
            },
            Operator::Add => values.iter().for_each(|v| data += *v),
            Operator::Mul => values.iter().for_each(|v| data *= *v),
            Operator::Min => values.iter().for_each(|v| data = min(data, *v)),
            Operator::Max => values.iter().for_each(|v| data = max(data, *v)),
        }

        // apply the value to all output wires
        for id in pins_out.0.iter() {
            if let Ok(mut pin) = next_query.get_mut(*id) {
                pin.0 |= data;
            }
        }
    }
}
