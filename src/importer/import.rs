use std::path::Path;
use num::{NumCast, PrimInt, Unsigned, traits::Zero};
use crate::matrix::*;
use crate::importer::{load_xraw_as_matrix, XRawMatrix, Voxel, VoxelType, ImportError};
use crate::schematic::{Schema, WireIndex, CompType, CompData};
use crate::circuit::*;


const THRESHOLD: usize = 3;

// read the xraw file at given location
pub fn load_xraw_file<P: AsRef<Path>>(path: P) -> Result<Schema, ImportError> {
    let matrix_result = match load_xraw_as_matrix(path) {
        Ok (r) => r,
        Err(e) => {return Err(e);}
    };

    match matrix_result {
        XRawMatrix::Ind8 (matrix) => Ok(convert_matrix_to_schema(&matrix, &|v| v == 0u8      , THRESHOLD, &|v, a| match_index(v as usize, a))),
        XRawMatrix::Ind16(matrix) => Ok(convert_matrix_to_schema(&matrix, &|v| v == 0xffffu16, THRESHOLD, &|v, a| match_index(v as usize, a))),
        XRawMatrix::Vox8 (matrix, voxel_type) => {

            Ok(Schema::new())
        },
        XRawMatrix::Vox16(matrix, voxel_type) => {

            Ok(Schema::new())
        },
        XRawMatrix::Vox32(matrix, voxel_type) => {

            Ok(Schema::new())
        },
        _ => Err(ImportError::Schema)
    }
}


// simply match indexes with component types
fn match_index(value: usize, volume: usize) -> ToBuild {
    // values from 1 to 16 are wires
    if 1 <= value && value <= 16 {
        return ToBuild::Wire((value - 1) as Channel);
    }
    match value {
        17 => ToBuild::Gate(Operator::Or  ),
        18 => ToBuild::Gate(Operator::And ),
        19 => ToBuild::Gate(Operator::Nor ),
        20 => ToBuild::Gate(Operator::Nand),
        21 => ToBuild::Gate(Operator::Add ),
        22 => ToBuild::Gate(Operator::Mul ),
        23 => ToBuild::Gate(Operator::Min ),
        24 => ToBuild::Gate(Operator::Max ),
        25 => ToBuild::Mux,
        26 => ToBuild::Demux(1),
        27 => ToBuild::Constant((volume - 4) as Data),
        28 => ToBuild::Bus,
        29 => ToBuild::Keyboard,
        _  => ToBuild::Empty,
    }
}


// return a function that detects if a voxel is empty based on the number of channels
fn get_empty_voxel_function<T: Copy + Zero>(voxel_type: VoxelType) -> Box<dyn Fn(Voxel<T>) -> bool> {
    match voxel_type {
        VoxelType::Cr   => Box::new(move |v| v.r().is_zero()),
        VoxelType::Crg  => Box::new(move |v| v.r().is_zero() && v.g().is_zero()),
        VoxelType::Crgb => Box::new(move |v| v.r().is_zero() && v.r().is_zero() && v.b().is_zero()),
        _               => Box::new(move |v| v.a().is_zero()),
    }
}