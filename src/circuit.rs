/**
 * logic components to build circuitry
*/

use bevy::prelude::*;


// operators available
pub enum Operator {
    Or,
    Nor,
    And,
    Nand,
    Add,
    Mul,
}


// represent a logic gate or an adder, multiplier
#[derive(Component)]
pub struct Gate {
    operator: Operator,
    input_wires: Vec<Entity>,
    output_wires: Vec<Entity>,
}

#[derive(Component)]
pub struct WirePreviousValue(u16);

#[derive(Component)]
pub struct WireNextValue(u16);

#[derive(Component)]
pub struct WireIndex(u8);


// apply operation to data from input wires
pub fn sys_tick_gate(
    gate_query: Query<&Gate>,
    wire_prev_query: Query<&WirePreviousValue>,
    mut wire_next_query: Query<&mut WireNextValue>
) {
    for gate in gate_query.iter() {

        // find the values of input wires
        let mut values = Vec::<u16>::with_capacity(gate.input_wires.len());
        for id in gate.input_wires.iter() {
            if let Ok(wire) = wire_prev_query.get(*id) {
                values.push(wire.0);
            }
        }

        // compute the output value
        let mut out: u16 = 0;
        match gate.operator {
            Operator::Or | Operator::Nor => for v in values.iter() {
                out |= v;
            },
            Operator::And | Operator::Nand => for v in values.iter() {
                out &= v;
            },
            Operator::Add => for v in values.iter() {
                out += v;
            },
            Operator::Mul => for v in values.iter() {
                out *= v;
            },
        }
        match gate.operator {
            Operator::Nor | Operator::Nand => out = !out,
            _ => ()
        }

        // apply the value to all output wires
        for id in gate.output_wires.iter() {
            if let Ok(mut wire) = wire_next_query.get_mut(*id) {
                wire.0 |= out;
            }
        }
    }
}


// reset the state of every wire
pub fn sys_tick_wire(
    mut wire_query: Query<(&mut WirePreviousValue, &mut WireNextValue)>
) {
    for (mut wire_prev, mut wire_next) in wire_query.iter_mut() {
        wire_prev.0 = wire_next.0;
        wire_next.0 = 0;
    }
}