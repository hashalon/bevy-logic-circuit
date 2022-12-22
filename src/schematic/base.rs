/**
 * represent a model to load, build and to display in bevy
 */
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::circuit::*;
use crate::math::*;


// define index for components
pub type Index = u32;

// indicate position of the model and model to use
#[derive(Serialize, Deserialize)]
pub struct ModelAttr {
    pub position  : Vec3i,
    pub mesh_index: Index,
}

// a wire of the schematic
#[derive(Serialize, Deserialize)]
pub struct SchemaWire {
    pub channel: Channel,
    pub model  : ModelAttr,
}

// the type of each element in the schematic
#[derive(Serialize, Deserialize)]
pub enum CompType {
    Bus,
    Mux,
    Demux (Data),
    Fixed (Data),
    Gate  (Operator),
    Input,
}

// an element of the schematic
#[derive(Serialize, Deserialize)]
pub struct SchemaComp {
    pub comp_type: CompType,
    pub pins_in  : Vec<Index>,
    pub pins_out : Vec<Index>,
    pub model    : ModelAttr,
}

pub fn convert_wire_list(indexes: &Vec<Index>, entities: &Vec<Entity>) -> Vec<Entity> {
    indexes.iter().map(|i| entities[*i as usize]).collect()
}