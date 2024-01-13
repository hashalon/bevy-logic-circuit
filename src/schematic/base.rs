use crate::circuit::*;
use crate::math::*;
/**
 * represent a model to load, build and to display in bevy
 */
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// define index for components
pub type Index = u32;

// indicate position of the model and model to use
#[derive(Serialize, Deserialize)]
pub struct ModelAttr {
    pub position: Vec3i,
    pub mesh_index: Index,
}

// a wire of the schematic
#[derive(Serialize, Deserialize)]
pub struct SchemaWire {
    pub channel: Channel,
    pub model: ModelAttr,
}

// the type of each element in the schematic
#[derive(Serialize, Deserialize)]
pub enum CompType {
    Bus,
    Mux,
    Demux(Data),
    Fixed(Data),
    Gate(Operator),
    Input,
}

// an element of the schematic
#[derive(Serialize, Deserialize)]
pub struct SchemaComp {
    pub comp_type: CompType,
    pub pins_in: Vec<Index>,
    pub pins_out: Vec<Index>,
    pub model: ModelAttr,
}

pub fn convert_wire_list(indexes: &[Index], entities: &[Entity]) -> Vec<Entity> {
    indexes.iter().map(|i| entities[*i as usize]).collect()
}
