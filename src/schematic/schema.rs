/**
 * represent a model to load, build and to display in bevy
 */
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs, path, io, fmt, error};
use crate::circuit::{Channel, NB_CHANNELS};
use crate::schematic::*;


// indicate position of the model and model to use
#[derive(Serialize, Deserialize, Resource)]
pub struct Schema {
    pub wires      : Vec<WireData>,
    pub components : Vec<CompData>,
    pub models     : Vec<ModelData>,
}


// error types when analyzing a schematic
#[derive(Debug)]
pub enum Error {
    WireChannel (usize, Channel),
    WireModel   (usize, WireIndex),
    CompModel   (usize, WireIndex),
    PinIn       (usize, usize),
    PinOut      (usize, usize),
}
impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::WireChannel (n, c) => write!(f, "Wire Channel Error at {} channel={}", n, c),
            Self::WireModel   (n, i)  => write!(f, "Wire Model Error at {}, index={}", n, i),
            Self::CompModel   (n, i)  => write!(f, "Component Model Error at {}, index={}", n, i),
            Self::PinIn       (n, i) => write!(f, "Pin Input Error at {}, {}", n, i),
            Self::PinOut      (n, i) => write!(f, "Pin Output Error at {}, {}", n, i),
        }
    }
}


impl Schema {
    pub fn new() -> Self {
        Self {
            wires      : Vec::<WireData> ::new(),
            components : Vec::<CompData> ::new(),
            models     : Vec::<ModelData>::new(),
        }
    }

    // check that the schema is valid before building the circuit
    pub fn verify(&self) -> Result<(), Vec<Error>> {
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
        for (i, elem) in self.components.iter().enumerate() {
            // check that associated model exists
            if elem.model_attr.index as usize >= nb_models {
                errors.push(Error::CompModel(i, elem.model_attr.index));
            }
            // check that inputs exist
            for pin in elem.pins_in.iter() {
                let j = *pin as usize;
                if j >= nb_wires {
                    errors.push(Error::PinIn(i, j));
                }
            }
            // check that outputs exist
            for pin in elem.pins_out.iter() {
                let j = *pin as usize;
                if j >= nb_wires {
                    errors.push(Error::PinOut(i, j));
                }
            }
        }

        // the schema is valid it can be used to generate a circuit
        return if errors.is_empty() {Ok(())} else {Err(errors)};
    }


    // load a file to generate a valid schematic
    pub fn load<P: AsRef<path::Path>>(path: P) -> Result<Self, Box<dyn error::Error>> {

        // try to open the file in read
        let mut file = match fs::File::open(path) {
            Ok(f)  => f,
            Err(e) => return Err(Box::new(e)),
        };

        // build a buffer to read the whole file data
        let mut buffer = Vec::new();
        if let Err(e) = io::Read::read_to_end(&mut file, &mut buffer) {
            return Err(Box::new(e));
        }

        // generate the schematic from the file
        let schema = match bincode::deserialize::<Schema>(&buffer) {
            Ok(s)  => s,
            Err(e) => return Err(Box::new(e)),
        };

        // schema has passed all the checks, can be returned
        Ok(schema)
    }

    // save to a file
    pub fn save<P: AsRef<path::Path>>(&self, path: P) -> Result<(), Box<dyn error::Error>> {

        // try to open the file in write
        let mut file = match fs::File::create(path) {
            Ok(f)  => f,
            Err(e) => return Err(Box::new(e)),
        };

        // try to serialize the schematic
        let buffer: Vec<u8> = match bincode::serialize(&self){
            Ok(b)  => b,
            Err(e) => return Err(Box::new(e)),
        };

        // write to the file
        if let Err(e) = io::Write::write(&mut file, &buffer) {
            return Err(Box::new(e));
        }

        Ok(())
    }
}


// build the whole circuit
pub fn build_circuit (mut commands: Commands, schema: Res<Schema>) {

    // generate list of wires
    let wires: Vec<Entity> = schema.wires.iter().map(|wire|
        commands
        .spawn(wire.bundle()).id()
    ).collect();

    // generate list of elements
    for comp in schema.components.iter() {
        /* TODO: could be used as soon as bevy support Bundle to be made into objects
        commands
        .spawn(comp.model_attr.bundle())
        .insert(comp.bundle(&wires));
        // */

        //* for now we have to implement a bundle fonction for each element type
        match comp.comp_type {
            CompType::Bus => {
                commands
                .spawn(comp.bundle_bus(&wires));
            }
            CompType::Mux => {
                commands
                .spawn(comp.bundle_mux(&wires));
            },
            CompType::Demux(value) => {
                commands
                .spawn(comp.bundle_demux(&wires, value));
            },
            CompType::Gate(op) => {
                commands
                .spawn(comp.bundle_gate(&wires, op));
            },
            CompType::Fixed(value) => {
                commands
                .spawn(comp.bundle_fixed(&wires, value));
            },
            CompType::Input => {
                commands
                .spawn(comp.bundle_input(&wires));
            },
        }; // */
    }
}


