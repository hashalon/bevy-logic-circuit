use std::{mem::size_of, fs::File, path::Path, io::{Read, BufRead, BufReader}};
use num::NumCast;
use crate::math::Vec3i;
use crate::voxel::Matrix;
use crate::importer::*;

// Voxel data
#[derive(Clone, Copy, Eq)]
pub struct Voxel<T>([T; 4]);


impl<T: Clone + Copy + Eq + NumCast> Voxel<T> {
    fn new(r: T, g: T, b: T, a: T) -> Self {
        Self([r, g, b, a])
    }
}

impl<T: Clone + Copy + Eq + NumCast + Default> Default for Voxel<T> {
    fn default() -> Self {
        Self([T::default(), T::default(), T::default(), T::default()])
    }
}

impl<T: Eq> PartialEq for Voxel<T> {
    #[inline]
    fn eq(&self, o: &Self) -> bool {
        for i in 0..4 {
            if self.0[i] != o.0[i] {return false}
        }
        true
    }
}


// read the header of the file before reading the content itself
// https://twitter.com/ephtracy/status/653721698328551424/photo/1
const HEADER_SIZE: usize = 24;


// specify the format of xraw file header
pub struct XRawHeader {
    magic_number            : String,
    color_channel_data_type : usize,
    color_channels_amount   : usize,
    bits_per_channel        : usize,
    bits_per_index          : usize,
    dimensions              : Vec3i,
    palette_colors_amount   : usize,
}


impl XRawHeader {

    // read only the header of the file
    pub fn load<R: BufRead>(reader: &mut R) -> Result<Self, LoadError> {

        // read the whole header into a buffer
        let mut buffer = [0u8; HEADER_SIZE];
        match reader.read(&mut buffer) {
            Ok(amount_bytes) => {
                if amount_bytes == HEADER_SIZE {
                    reader.consume(HEADER_SIZE);
                    return Ok(Self {
                        magic_number            : read_string(&buffer, 0, 4),
                        color_channel_data_type : buffer[4] as usize,
                        color_channels_amount   : buffer[5] as usize,
                        bits_per_channel        : buffer[6] as usize,
                        bits_per_index          : buffer[7] as usize,
                        dimensions              : read_vec3i_from_u32s(&buffer, 8),
                        palette_colors_amount   : read_u32(&buffer, 20) as usize,
                    });
                } else {
                    return Err(LoadError::Insufficient(HEADER_SIZE, amount_bytes));
                }
            },
            Err(err) => return Err(LoadError::ReadFile(err))
        }
    }
}


// indicate which type of matrix has been returned
pub enum XRawMatrix {
    Ind8 (Matrix<u8 >),
    Ind16(Matrix<u16>),
    Vox8 (Matrix<Voxel<u8 >>),
    Vox16(Matrix<Voxel<u16>>),
    Vox32(Matrix<Voxel<u32>>),
}

// indicate the type of error encountered while trying to load a xraw file
pub enum XRawLoadError {
    ReadFile,
    ReadHeader,
    ReadContent,
    ReadMatrix,
}


// load the file and get a matrix with the most suited type
pub fn load_file<P: AsRef<Path>>(path: P) -> Result<XRawMatrix, XRawLoadError> {
    match File::open(path) {
        Ok(file) => {
            let file_size = file.metadata().unwrap().len() as usize;

            let mut reader = BufReader::new(file);
            if let Ok(header) = XRawHeader::load(&mut reader) {
                let mut buffer = Vec::<u8>::with_capacity(file_size);
                if let Ok(_) = reader.read_to_end(&mut buffer) {
                    match header.bits_per_index {
                        8  => Ok(XRawMatrix::Ind8 (load_matrix_u8( &buffer, header.dimensions))),
                        16 => Ok(XRawMatrix::Ind16(load_matrix_u16(&buffer, header.dimensions))),
                        _  => {
                            match header.bits_per_channel {
                                8  => Ok(XRawMatrix::Vox8 (load_matrix_voxel::<u8 >(&buffer, header.dimensions, header.color_channels_amount))),
                                16 => Ok(XRawMatrix::Vox16(load_matrix_voxel::<u16>(&buffer, header.dimensions, header.color_channels_amount))),
                                32 => Ok(XRawMatrix::Vox32(load_matrix_voxel::<u32>(&buffer, header.dimensions, header.color_channels_amount))),
                                _  => Err(XRawLoadError::ReadMatrix)
                            }
                        }
                    }
                } else {
                    Err(XRawLoadError::ReadContent)
                }
            } else {
                Err(XRawLoadError::ReadHeader)
            }
        },
        Err(err) => {Err(XRawLoadError::ReadFile)}
    }
}

// load the matrix containing u8 indexes
pub fn load_matrix_u8(buffer: &Vec<u8>, size: Vec3i) -> Matrix<u8> {
    let mut matrix = Matrix::<u8>::new(size, 0u8);
    let mut index  = 0;
    for cell in matrix.data.iter_mut() {
        *cell = buffer[index];
        index += 1;
    }
    return matrix;
}

// load the matrix containing u16 indexes
pub fn load_matrix_u16(buffer: &Vec<u8>, size: Vec3i) -> Matrix<u16> {
    let mut matrix = Matrix::<u16>::new(size, 0xffffu16);
    let mut index  = 0;
    for cell in matrix.data.iter_mut() {
        *cell = read_u16(&buffer, index);
        index += 2;
    }
    return matrix;
}

// load the matrix of voxels
pub fn load_matrix_voxel<T: Clone + Copy + Eq + NumCast + Default>(
    buffer: &Vec<u8>, size: Vec3i, channels_amount: usize) 
-> Matrix<Voxel<T>> {

    let empty = Voxel::default();
    let mut matrix = Matrix::<Voxel<T>>::new(size, empty);

    let mut index = 0;
    match size_of::<T>() {
        1 => {
            for cell in matrix.data.iter_mut() {
                *cell = empty;
                for i in 0..channels_amount {
                    (*cell).0[i] = T::from(buffer[index]).unwrap();
                    index += 1;
                }
            }
        },
        2 => {
            for cell in matrix.data.iter_mut() {
                *cell = empty;
                for i in 0..channels_amount {
                    (*cell).0[i] = T::from(read_u16(&buffer, index)).unwrap();
                    index += 2;
                }
            }
        },
        4 => {
            for cell in matrix.data.iter_mut() {
                *cell = empty;
                for i in 0..channels_amount {
                    (*cell).0[i] = T::from(read_u32(&buffer, index)).unwrap();
                    index += 4;
                }
            }
        },
        _ => {}
    }
    return matrix;
}
