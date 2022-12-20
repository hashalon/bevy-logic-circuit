/**
 * represent a model to load, build and to display in bevy
 */
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schematic::*;
use crate::circuit::*;


// define index for components
pub type WireIndex = u32;

// a wire of the schematic
#[derive(Serialize, Deserialize)]
pub struct WireData {
    pub channel    : Channel,
    pub model_attr : ModelAttr,
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
pub struct CompData {
    pub comp_type  : CompType,
    pub pins_in    : Vec<WireIndex>,
    pub pins_out   : Vec<WireIndex>,
    pub model_attr : ModelAttr,
}

impl WireData {
    pub fn bundle(&self) -> WireBundle {
        WireBundle {
            index: PinChannel   (self.channel),
            prev : DataPrevious (0),
            next : DataNext     (0)
        }
    }
}


macro_rules! make_bundle {
    {$self:ident, $wires:ident, $bundle:ident, $value:expr} => {
        $bundle {
            comp    : $value,
            pins_in : PinsIn  (convert_list(&$self.pins_in , $wires)),
            pins_out: PinsOut (convert_list(&$self.pins_out, $wires)),
        }
    };
}

fn convert_list(indexes: &Vec<WireIndex>, entities: &Vec<Entity>) -> Vec<Entity> {
    indexes.iter().map(|i| entities[*i as usize]).collect()
}


impl CompData {
    pub fn bundle_bus(&self, wires: &Vec<Entity>) -> BusBundle {
        make_bundle!(self, wires, BusBundle, Bus {})
    }
    pub fn bundle_mux(&self, wires: &Vec<Entity>) -> MuxBundle {
        make_bundle!(self, wires, MuxBundle, Mux {})
    }
    pub fn bundle_demux(&self, wires: &Vec<Entity>, value: Data) -> DemuxBundle {
        make_bundle!(self, wires, DemuxBundle, Demux(value))
    }
    pub fn bundle_gate(&self, wires: &Vec<Entity>, op: Operator) -> GateBundle {
        make_bundle!(self, wires, GateBundle, op)
    }
    pub fn bundle_fixed(&self, wires: &Vec<Entity>, value: Data) -> FixedBundle {
        FixedBundle {
            comp: Fixed (value),
            pins_out: PinsOut (convert_list(&self.pins_out, wires)),
        }
    }
    pub fn bundle_input(&self, wires: &Vec<Entity>) -> InputBundle {
        InputBundle {
            comp: Connector {},
            pins_out: PinsOut (convert_list(&self.pins_out, wires)),
        }
    }
}