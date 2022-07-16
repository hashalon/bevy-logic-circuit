use std::{fs::File, path::Path, io::Read};
use crate::math::Vec3i;
use crate::voxel::Matrix;
use crate::importer::*;

// read the header of the file before reading the content itself
// https://twitter.com/ephtracy/status/653721698328551424/photo/1
const HEADER_SIZE: usize = 24;


// specify the format of xraw file header
pub struct XRawHeader {
    magic_number            : String,
    color_channel_data_type : u8,
    color_channels_amount   : u8,
    bits_per_channel        : u8,
    bits_per_index          : u8,
    dimensions              : Vec3i,
    palette_colors_amount   : u32,
}


impl XRawHeader {

    // read only the header of the file
    pub fn load<R: Read>(reader: &mut R) -> Result<Self, LoadError> {

        // read the whole header into a buffer
        let mut buffer = Vec::<u8>::with_capacity(HEADER_SIZE);
        let mut header = reader.take(HEADER_SIZE as u64);
        match header.read_to_end(&mut buffer) {
            Ok(amount_bytes) => {
                if amount_bytes >= HEADER_SIZE {
                    return Ok(Self {
                        magic_number            : read_string(&buffer, 0, 4),
                        color_channel_data_type : buffer[4],
                        color_channels_amount   : buffer[5],
                        bits_per_channel        : buffer[6],
                        bits_per_index          : buffer[7],
                        dimensions              : read_vec3i_from_u32s(&buffer, 8),
                        palette_colors_amount   : read_u32(&buffer, 20),
                    });
                } else {
                    return Err(LoadError::Insufficient(HEADER_SIZE, amount_bytes));
                }
            },
            Err(err) => return Err(LoadError::ReadFile(err))
        }
    }
}
