/**
 * represent a model to load, build and to display in bevy
 */
use crate::schematic::*;
use crate::circuit::{Channel, Data, Operator};
use serde::{Deserialize, Serialize};


// the type of each element in the schematic
#[derive(Serialize, Deserialize)]
pub enum Type {
    Constant(Data),
    Gate(Operator),
    Mux,
    Demux(Data),
    Keyboard,
}

// an element of the schematic
#[derive(Serialize, Deserialize)]
pub struct Element {
    pub type_elem  : Type,
    pub pins_in    : Vec<Index>,
    pub pins_out   : Vec<Index>,
    pub model_attr : ModelAttr,
}

impl Element {

}

// a wire of the schematic
#[derive(Serialize, Deserialize)]
pub struct Wire {
    pub channel    : Channel,
    pub model_attr : ModelAttr,
}

impl Wire {

}
