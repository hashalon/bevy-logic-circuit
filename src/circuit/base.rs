/**
 * structs to connect components together
*/
use bevy::prelude::*;


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



// reset the state of every wire
pub fn sys_reset(
    mut query: Query<(&mut DataPrevious, &mut DataNext)>
) {
    query.for_each_mut(|(mut wire_prev, mut wire_next)| {
        wire_prev.0 = wire_next.0;
        wire_next.0 = 0;
    });
}
