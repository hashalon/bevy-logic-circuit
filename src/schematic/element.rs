/**
 * represent a model to load, build and to display in bevy
 */
use bevy::prelude::*;
use crate::schematic::*;
use crate::circuit::*;
use serde::{Deserialize, Serialize};


// a wire of the schematic
#[derive(Serialize, Deserialize)]
pub struct Wire {
    pub channel    : Channel,
    pub model_attr : ModelAttr,
}

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
    pub pins_in    : Vec<CompIndex>,
    pub pins_out   : Vec<CompIndex>,
    pub model_attr : ModelAttr,
}

impl Wire {
    pub fn bundle(&self) -> WireBundle {
        WireBundle {
            index: PinChannel(self.channel),
            prev : DataPrevious(0),
            next : DataNext    (0)
        }
    }
}

impl Element {
    /* TODO: could be used as soon as bevy support Bundle to be made into objects
    pub fn bundle(&self, wires: &Vec<Entity>) -> dyn Bundle {
        let pins_in  = PinsIn (convert_list(&self.pins_in , wires));
        let pins_out = PinsOut(convert_list(&self.pins_out, wires));

        match self.type_elem {
            Type::Constant(value) => BundleConst {
                comp: Constant(value),
                pins_out,
            },
            Type::Gate(op) => BundleGate {
                operator: op,
                pins_in ,
                pins_out,
            },
            Type::Mux => BundleMux {
                comp: Mux {},
                pins_in ,
                pins_out,
            },
            Type::Demux(value) => BundleDemux {
                comp: Demux(value),
                pins_in ,
                pins_out,
            },
            Type::Keyboard => BundleKeyboard {
                comp: KeyboardConnector {},
                pins_out,
            },
        }
    } // */

    pub fn bundle_const(&self, wires: &Vec<Entity>, value: Data) -> BundleConst {
        BundleConst {
            comp: Constant(value),
            pins_out: PinsOut(convert_list(&self.pins_out, wires)),
        }
    }
    pub fn bundle_gate(&self, wires: &Vec<Entity>, op: Operator) -> BundleGate {
        BundleGate {
            operator: op,
            pins_in : PinsIn (convert_list(&self.pins_in , wires)),
            pins_out: PinsOut(convert_list(&self.pins_out, wires)),
        }
    }
    pub fn bundle_mux(&self, wires: &Vec<Entity>) -> BundleMux {
        BundleMux {
            comp: Mux {},
            pins_in : PinsIn (convert_list(&self.pins_in , wires)),
            pins_out: PinsOut(convert_list(&self.pins_out, wires)),
        }
    }
    pub fn bundle_demux(&self, wires: &Vec<Entity>, value: Data) -> BundleDemux {
        BundleDemux {
            comp: Demux(value),
            pins_in : PinsIn (convert_list(&self.pins_in , wires)),
            pins_out: PinsOut(convert_list(&self.pins_out, wires)),
        }
    }
    pub fn bundle_keyboard(&self, wires: &Vec<Entity>) -> BundleKeyboard {
        BundleKeyboard {
            comp: KeyboardConnector {},
            pins_out: PinsOut(convert_list(&self.pins_out, wires)),
        }
    }
}

fn convert_list(indexes: &Vec<CompIndex>, entities: &Vec<Entity>) -> Vec<Entity> {
    indexes.iter().map(|i| entities[*i as usize]).collect()
}
