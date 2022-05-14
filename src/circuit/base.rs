/**
 * structs to connect components together
*/
use bevy::prelude::*;


pub const CHANNELS:  usize = 16;
pub const DATA_SIZE: usize = std::mem::size_of::<Data>();

// Data that is transmitted over wires
pub type Data = u16;

// data on the wire on the previous tick
#[derive(Component)]
pub struct DataPrevious(pub Data);

// data on the wire on the next tick
#[derive(Component)]
pub struct DataNext(pub Data);


// index/color of a wire
#[derive(Component)]
pub struct PinIndex(pub u8);


// inputs entering a component
#[derive(Component)]
pub struct Inputs(pub Vec<Entity>);

// outputs leaving a component
#[derive(Component)]
pub struct Outputs(pub Vec<Entity>);


// constant input value
#[derive(Component)]
pub struct Constant(pub Data);



// reset the state of every wire
pub fn sys_reset(
    mut query: Query<(&mut DataPrevious, &mut DataNext)>
) {
    query.for_each_mut(|(mut wire_prev, mut wire_next)| {
        wire_prev.0 = wire_next.0;
        wire_next.0 = 0;
    });
}


// simply apply the constant
pub fn sys_tick(
    comp_query: Query<(&Constant, &Outputs)>,
    mut next_query: Query<&mut DataNext>
) {
    for (constant, pins_out) in comp_query.iter() {

        // apply the value to all output wires
        for id in pins_out.0.iter() {
            if let Ok(mut pin) = next_query.get_mut(*id) {
                pin.0 |= constant.0;
            }
        }
    }
}
