/**
 * represent a model to load, build and to display in bevy
 */
use bevy::prelude::*;
use crate::circuit::{Channel, NB_CHANNELS};
use crate::schematic::*;
use serde::{Deserialize, Serialize};

// indicate position of the model and model to use
#[derive(Serialize, Deserialize)]
pub struct Schema {
    pub wires    : Vec<Wire>,
    pub elements : Vec<Element>,
    pub models   : Vec<ModelData>,
}


impl Schema {
    pub fn new() -> Self {
        Self {
            wires    : Vec::<Wire     >::new(),
            elements : Vec::<Element  >::new(),
            models   : Vec::<ModelData>::new(),
        }
    }

    // check that the schema is valid before building the circuit
    pub fn verify(&self) -> Result<bool, Vec<Error>> {
        let mut errors = Vec::<Error>::new();

        let nb_wires  = self.wires .len();
        let nb_models = self.models.len();

        // check that wires are valid
        for (i, wire) in self.wires.iter().enumerate() {
            // check that the channel of the wire is valid
            if wire.channel as usize >= NB_CHANNELS {
                errors.push(Error::WireChannel(i, wire.channel));
            }
            // check that associated model exists
            if wire.model_attr.index as usize >= nb_models {
                errors.push(Error::WireModel(i, wire.model_attr.index));
            }
        }

        // check that all elements are valid
        for (i, elem) in self.elements.iter().enumerate() {
            // check that associated model exists
            if elem.model_attr.index as usize >= nb_models {
                errors.push(Error::ElemModel(i, elem.model_attr.index));
            }
            // check that inputs exist
            for pin in elem.pins_in.iter() {
                let j = *pin as usize;
                if j >= nb_wires {
                    errors.push(Error::ElemPinIn(i, j));
                }
            }
            // check that outputs exist
            for pin in elem.pins_out.iter() {
                let j = *pin as usize;
                if j >= nb_wires {
                    errors.push(Error::ElemPinOut(i, j));
                }
            }
        }

        // the schema is valid it can be used to generate a circuit
        return if errors.is_empty() {Ok(true)} else {Err(errors)};
    }

    pub fn load(&self) {

    }

    /* save to a file
    pub fn save(&self) {
        let bin: Vec<u8> = bincode::serialize(&self).unwrap();
    } // */

    /* load from a file
    pub fn load(data: &Vec<u8>) {
        let value: Self = bincode::deserialize(data).unwrap();
    } // */

}


// build the whole circuit
pub fn build_circuit (mut commands: Commands, schema: Res<Schema>) {

    // generate list of wires
    let wires: Vec<Entity> = schema.wires.iter().map(|wire|
        commands
        .spawn_bundle (wire.model_attr.bundle())
        .insert_bundle(wire.bundle()).id()
    ).collect();

    // generate list of elements
    for elem in schema.elements.iter() {
        /* TODO: could be used as soon as bevy support Bundle to be made into objects
        commands
        .spawn_bundle(elem.model_attr.bundle())
        .insert_bundle(elem.bundle(&wires));
        // */

        // for now we have to implement a bundle fonction for each element type
        match elem.type_elem {
            Type::Constant(value) => {
                commands
                .spawn_bundle (elem.model_attr.bundle())
                .insert_bundle(elem.bundle_const(&wires, value));
            },
            Type::Gate(op) => {
                commands
                .spawn_bundle (elem.model_attr.bundle())
                .insert_bundle(elem.bundle_gate(&wires, op));
            },
            Type::Mux => {
                commands
                .spawn_bundle (elem.model_attr.bundle())
                .insert_bundle(elem.bundle_mux(&wires));
            },
            Type::Demux(value) => {
                commands
                .spawn_bundle (elem.model_attr.bundle())
                .insert_bundle(elem.bundle_demux(&wires, value));
            },
            Type::Keyboard => {
                commands
                .spawn_bundle (elem.model_attr.bundle())
                .insert_bundle(elem.bundle_keyboard(&wires));
            },
        };
    }
}


// error types when analyzing a schematic
pub enum Error {
    WireChannel(usize, Channel),
    WireModel  (usize, Index),
    ElemModel  (usize, Index),
    ElemPinIn  (usize, usize),
    ElemPinOut (usize, usize),
}

impl Error {
    pub fn message (&self) -> &str {
        match self {
            Self::WireChannel(id, chann) => "",
            Self::WireModel  (id, model) => "",
            Self::ElemModel  (id, model) => "",
            Self::ElemPinIn  (id, pin  ) => "",
            Self::ElemPinOut (id, pin  ) => "",
        }
    }
}
