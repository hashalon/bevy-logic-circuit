use std::{fs::File, path::Path, io::{BufRead, BufReader}};
use crate::math::Vec3i;
use crate::voxel::Matrix;
use crate::importer::*;

// Voxel data
#[derive(Clone, Copy, Eq)]
pub struct Voxel([u32; 4]);


impl Voxel {
    fn new(r: u32, g: u32, b: u32, a: u32) -> Self {
        Self([r, g, b, a])
    }

    fn void() -> Self {
        Self([0, 0, 0, 0])
    }
}

impl PartialEq for Voxel {
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


pub fn load_file<P: AsRef<Path>>(path: P) {
    match File::open(path) {
        Ok(file) => {
            let file_size = file.metadata().unwrap().len() as usize;

            let mut reader = BufReader::new(file);
            if let Ok(header) = XRawHeader::load(&mut reader) {
                let mut buffer = Vec::<u8>::with_capacity(file_size);
                //file.read_to_end(&mut buffer);

                let size = header.dimensions;
                match header.bits_per_index {
                    8  => {load_matrix(&buffer, size, 0u8      , 1, 1, &|buf, index, _| {buf[index]});},
                    16 => {load_matrix(&buffer, size, 0xffffu16, 2, 1, &|buf, index, _| {read_u16(buf, index)});},
                    _  => {
                        let data_size = header.color_channels_amount * header.bits_per_channel / 8;
                        let func = voxel_gen_func(header.bits_per_channel);
                        load_matrix(&buffer, size, Voxel::void(), data_size, header.color_channels_amount, &func);
                    }
                };
            }
        },
        Err(err) => {}
    }
}

pub fn generate_matrix_u8(buffer: &Vec<u8>, size: Vec3i) -> Matrix<u8> {
    load_matrix(&buffer, size, 0u8, 1, 1, &|buf, index, _| {buf[index]})
}

pub fn generate_matrix_u16(buffer: &Vec<u8>, size: Vec3i) -> Matrix<u16> {
    load_matrix(&buffer, size, 0xffffu16, 2, 1, &|buf, index, _| {read_u16(buf, index)})
}

pub fn generate_matrix_voxel(buffer: &Vec<u8>, size: Vec3i, color_channels_amount: usize, bits_per_channel: usize) -> Matrix<Voxel> {
    load_matrix(&buffer, size, Voxel::void(), 
    color_channels_amount * bits_per_channel / 8, 
    color_channels_amount, &voxel_gen_func(bits_per_channel))
}


// load data into the buffer and convert it into a matrix
fn load_matrix<T: Clone + Copy + Eq>(buffer: &[u8], size: Vec3i, empty: T, 
    data_size: usize, data_elem: usize, func: &dyn Fn(&[u8], usize, usize) -> T) 
-> Matrix<T> {

    let mut matrix = Matrix::<T>::new(size, empty);
    let mut index  = 0;
    for cell in matrix.data.iter_mut() {
        *cell = func(buffer, index, data_elem);
        index += data_size;
    }
    return matrix;
}


// get a lambda function for reading the given type of voxel
fn voxel_gen_func(data_type_size: usize) -> Box<dyn Fn(&[u8], usize, usize) -> Voxel> {
    match data_type_size {
        8 => {
            Box::new(|buffer, index, nb_channels| {
                let mut vox = Voxel::void();
                for i in 0..nb_channels {
                    vox.0[i] = buffer[index + i] as u32;
                }
                vox
            })
        },
        16 => {
            Box::new(|buffer, index, nb_channels| {
                let mut vox = Voxel::void();
                for i in 0..nb_channels {
                    vox.0[i] = read_u16(buffer, index + i) as u32;
                }
                vox
            })
        },
        32 => {
            Box::new(|buffer, index, nb_channels| {
                let mut vox = Voxel::void();
                for i in 0..nb_channels {
                    vox.0[i] = read_u32(buffer, index + i);
                }
                vox
            })
        },
        _ => Box::new(|_, _, _| {Voxel::void()})
    }
}