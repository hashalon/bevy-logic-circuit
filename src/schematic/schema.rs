/**
 * represent a model to load, build and to display in bevy
 */
use crate::circuit::{Channel, NB_CHANNELS};
use crate::schematic::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write, Error};
use std::fs::File;
use std::path::PathBuf;


// indicate position of the model and model to use
#[derive(Serialize, Deserialize)]
pub struct Schema {
    pub wires    : Vec<Wire>,
    pub elements : Vec<Element>,
    pub models   : Vec<ModelData>,
}


// error types when analyzing a schematic
pub enum ErrorSchema {
    WireChannel(usize, Channel),
    WireModel  (usize, Index),
    ElemModel  (usize, Index),
    ElemPinIn  (usize, usize),
    ElemPinOut (usize, usize),
}

impl ErrorSchema {
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


// error types when analyzing a schematic
pub enum ErrorFile {
    Unknown,
    Open (Error),
    Read (Error),
    Write(Error),
    Serialize,
    Deserialize,
}

impl ErrorFile {
    pub fn message (&self) -> &str {
        match self {
            Self::Unknown     => "Could not identify file format",
            Self::Open  (e)   => "Could not open input file.",
            Self::Read  (e)   => "Could not read input file.",
            Self::Write (e)   => "Could not write input file.",
            Self::Serialize   => "Could not serialize input file",
            Self::Deserialize => "Could not deserialize input file",
        }
    }
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
    pub fn verify(&self) -> Result<(), Vec<ErrorSchema>> {
        let mut errors = Vec::<ErrorSchema>::new();

        let nb_wires  = self.wires .len();
        let nb_models = self.models.len();

        // check that wires are valid
        for (i, wire) in self.wires.iter().enumerate() {
            // check that the channel of the wire is valid
            if wire.channel as usize >= NB_CHANNELS {
                errors.push(ErrorSchema::WireChannel(i, wire.channel));
            }
            // check that associated model exists
            if wire.model_attr.index as usize >= nb_models {
                errors.push(ErrorSchema::WireModel(i, wire.model_attr.index));
            }
        }

        // check that all elements are valid
        for (i, elem) in self.elements.iter().enumerate() {
            // check that associated model exists
            if elem.model_attr.index as usize >= nb_models {
                errors.push(ErrorSchema::ElemModel(i, elem.model_attr.index));
            }
            // check that inputs exist
            for pin in elem.pins_in.iter() {
                let j = *pin as usize;
                if j >= nb_wires {
                    errors.push(ErrorSchema::ElemPinIn(i, j));
                }
            }
            // check that outputs exist
            for pin in elem.pins_out.iter() {
                let j = *pin as usize;
                if j >= nb_wires {
                    errors.push(ErrorSchema::ElemPinOut(i, j));
                }
            }
        }

        // the schema is valid it can be used to generate a circuit
        return if errors.is_empty() {Ok(())} else {Err(errors)};
    }


    // load a file to generate a valid schematic
    pub fn load(file_path: PathBuf) -> Result<Self, ErrorFile> {

        // try to open the file in read
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => return Err(ErrorFile::Open(e)),
        };

        // build a buffer to read the whole file data
        let mut buffer = Vec::new();
        if let Err(e) = file.read_to_end(&mut buffer) {
            return Err(ErrorFile::Read(e));
        }

        // generate the schematic from the file
        let schema = match bincode::deserialize::<Schema>(&buffer) {
            Ok(s) => s,
            Err(_) => return Err(ErrorFile::Deserialize),
        };

        // schema has passed all the checks, can be returned
        Ok(schema)
    }

    // save to a file
    pub fn save(&self, file_path: PathBuf) -> Result<(), ErrorFile> {

        // try to open the file in write
        let mut file = match File::create(file_path) {
            Ok(f) => f,
            Err(e) => return Err(ErrorFile::Open(e)),
        };

        // try to serialize the schematic
        let buffer: Vec<u8> = match bincode::serialize(&self){
            Ok(b) => b,
            Err(_) => return Err(ErrorFile::Serialize),
        };

        // write to the file
        if let Err(e) = file.write(&buffer) {
            return Err(ErrorFile::Write(e));
        }

        Ok(())
    }
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


