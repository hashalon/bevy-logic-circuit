/**
 * represent a model to load, build and to display in bevy
 */
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs, path, io, fmt, error};
use crate::circuit::*;
use crate::schematic::*;


// indicate position of the model and model to use
#[derive(Serialize, Deserialize, Resource)]
pub struct Schema {
    wires  : Vec<SchemaWire>,
    comps  : Vec<SchemaComp>,
    models : Vec<Model>,
}


// error types when analyzing a schematic
#[derive(Debug)]
pub enum Error {
    WireChannel (usize, Channel),
    WireModel   (usize, Index),
    CompModel   (usize, Index),
    PinIn       (usize, usize),
    PinOut      (usize, usize),
}
impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::WireChannel (n, c) => write!(f, "Wire Channel Error at {} channel={}", n, c),
            Self::WireModel   (n, i) => write!(f, "Wire Model Error at {}, index={}", n, i),
            Self::CompModel   (n, i) => write!(f, "Component Model Error at {}, index={}", n, i),
            Self::PinIn       (n, i) => write!(f, "Pin Input Error at {}, {}", n, i),
            Self::PinOut      (n, i) => write!(f, "Pin Output Error at {}, {}", n, i),
        }
    }
}


impl Default for Schema {
    fn default() -> Self {
        Self {
            wires : Vec::default(),
            comps : Vec::default(),
            models: Vec::default()
        }
    }
}


impl Schema {
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
            if wire.model.mesh_index as usize >= nb_models {
                errors.push(Error::WireModel(i, wire.model.mesh_index));
            }
        }

        // check that all elements are valid
        for (i, elem) in self.comps.iter().enumerate() {
            // check that associated model exists
            if elem.model.mesh_index as usize >= nb_models {
                errors.push(Error::CompModel(i, elem.model.mesh_index));
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
pub fn build_circuit (
    mut commands : Commands, 
    mut meshes   : ResMut<Assets<Mesh>>,
    materials    : Res<MaterialStore>,
    schema       : Res<Schema>
) {
    // store generated mesh handles in a simple vector
    let models: Vec<Handle<Mesh>> = schema.models.iter().map(|model|
        meshes.add(model.to_mesh())
    ).collect();
    
    // generate list of wires
    let wires: Vec<Entity> = schema.wires.iter().map(|wire|
        commands.spawn((
            PinChannel (wire.channel), 
            DataPrev   (0), 
            DataNext   (0)
        )).id()
    ).collect();

    // generate list of elements
    for comp in schema.comps.iter() {
        let pins_in  = PinsIn(convert_wire_list(&comp.pins_in , &wires));
        let pins_out = PinsOut(convert_wire_list(&comp.pins_out, &wires));

        match comp.comp_type {
            CompType::Gate (op) => {
                commands.spawn((op, pins_in, pins_out));
            },
            CompType::Mux => {
                commands.spawn((CompMux {}, pins_in, pins_out));
            },
            CompType::Demux (val) => {
                commands.spawn((CompDemux (val), pins_in, pins_out));
            },
            CompType::Fixed (val) => {
                commands.spawn((CompFixed (val), pins_out));
            },
            CompType::Bus => {
                commands.spawn((CompIOBus {}, pins_in, pins_out));
            },
            CompType::Input => {
                commands.spawn((CompInput {}, pins_out));
            },
        }
    }
}


