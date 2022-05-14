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
pub struct PinsIn(pub Vec<Entity>);

// outputs leaving a component
#[derive(Component)]
pub struct PinsOut(pub Vec<Entity>);


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


pub enum CompType {
    Constant,
    Gate(Operator),
    Mux,
    Demux,
    Keyboard,
}


