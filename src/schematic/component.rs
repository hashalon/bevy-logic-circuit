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
    Demux(Data),
    Constant(Data),
    Gate(Operator),
    Keyboard,
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
            index: PinChannel(self.channel),
            prev : DataPrevious(0),
            next : DataNext    (0)
        }
    }
}

impl CompData {
    /* TODO: could be used as soon as bevy support Bundle to be made into objects
    pub fn bundle(&self, wires: &Vec<Entity>) -> Box<&dyn Bundle> {
        let pins_in  = PinsIn (convert_list(&self.pins_in , wires));
        let pins_out = PinsOut(convert_list(&self.pins_out, wires));

        match self.comp_type {
            CompType::Bus => Box::new(move BundleBus {
                comp: Bus {},
                pins_in ,
                pins_out,
            }),
            CompType::Mux => Box::new(move BundleMux {
                comp: Mux {},
                pins_in ,
                pins_out,
            }),
            CompType::Demux(value) => Box::new(move BundleDemux {
                comp: Demux(value),
                pins_in ,
                pins_out,
            }),
            CompType::Constant(value) => Box::new(move BundleConst {
                comp: Constant(value),
                pins_out,
            }),
            CompType::Gate(op) => Box::new(move BundleGate {
                operator: op,
                pins_in ,
                pins_out,
            }),
            CompType::Keyboard => Box::new(move BundleKeyboard {
                comp: KeyboardConnector {},
                pins_out,
            }),
        }
    } // */

    pub fn bundle_bus(&self, wires: &Vec<Entity>) -> BundleBus {
        BundleBus {
            comp: Bus {},
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
    pub fn bundle_keyboard(&self, wires: &Vec<Entity>) -> BundleKeyboard {
        BundleKeyboard {
            comp: KeyboardConnector {},
            pins_out: PinsOut(convert_list(&self.pins_out, wires)),
        }
    }
}

fn convert_list(indexes: &Vec<WireIndex>, entities: &Vec<Entity>) -> Vec<Entity> {
    indexes.iter().map(|i| entities[*i as usize]).collect()
}
