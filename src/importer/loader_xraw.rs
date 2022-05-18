use std::fs::File;
use std::io::BufReader;
use crate::math::*;
use crate::importer::voxel::*;


// read data from the binary file
type Byte  = [u8; 1];
type Short = [u8; 2];
type Word  = [u8; 4];


pub fn load_xraw() -> Model {
    // https://dev.to/dandyvica/different-ways-of-reading-files-in-rust-2n30
    if let Ok(file) = File::open("some_file.txt") {
        let mut reader = BufReader::new(file);

        let mut file_type:         Word;
        let mut data_type:         Byte;
        let mut color_channels:    Byte;
        let mut bits_per_channels: Byte;
        let mut bits_per_indexes:  Byte;
        let mut size_x:            Word;
        let mut size_y:            Word;
        let mut size_z:            Word;
        let mut nb_colors:         Word;

        /*
        reader.read(file_type);
        reader.read(data_type);
        reader.read(color_channels);
        reader.read(bits_per_channels);
        reader.read(bits_per_indexes);
        reader.read(size_x);
        reader.read(size_y);
        reader.read(size_z);
        reader.read(nb_colors);
        */
    }
    let size = Vec3i::new(0, 0, 0);
    let mut model = Model::prepare(size);


    return model;
}