use std::{str::from_utf8, io};
use crate::math::Vec3i;

// define the various types of error we can encounter when loading a file
pub enum LoadError {
    ReadFile(io::Error),
    Insufficient(usize, usize),
}


// read a string
#[inline]
pub fn read_string(buffer: &Vec<u8>, index: usize, length: usize) -> String {
    from_utf8(&buffer[index..(index + length)]).unwrap().to_string()
}

// read a number
#[inline]
pub fn read_u16(buffer: &Vec<u8>, index: usize) -> u16 {
    u16::from_le_bytes((&buffer[index..(index + 2)]).try_into().unwrap())
}

// read a number
#[inline]
pub fn read_u32(buffer: &Vec<u8>, index: usize) -> u32 {
    u32::from_le_bytes((&buffer[index..(index + 4)]).try_into().unwrap())
}

// read a number
#[inline]
pub fn read_u64(buffer: &Vec<u8>, index: usize) -> u64 {
    u64::from_le_bytes((&buffer[index..(index + 8)]).try_into().unwrap())
}


// read a 3D vector
#[inline]
pub fn read_vec3i_from_u8s(buffer: &Vec<u8>, index: usize) -> Vec3i {
    Vec3i::new(
        buffer[index    ] as usize,
        buffer[index + 1] as usize,
        buffer[index + 2] as usize
    )
}

// read a 3D vector
#[inline]
pub fn read_vec3i_from_u16s(buffer: &Vec<u8>, index: usize) -> Vec3i {
    Vec3i::new(
        read_u16(&buffer, index    ) as usize,
        read_u16(&buffer, index + 2) as usize,
        read_u16(&buffer, index + 4) as usize
    )
}

// read a 3D vector
#[inline]
pub fn read_vec3i_from_u32s(buffer: &Vec<u8>, index: usize) -> Vec3i {
    Vec3i::new(
        read_u32(&buffer, index    ) as usize,
        read_u32(&buffer, index + 4) as usize,
        read_u32(&buffer, index + 8) as usize
    )
}

// read a 3D vector
#[inline]
pub fn read_vec3i_from_u64s(buffer: &Vec<u8>, index: usize) -> Vec3i {
    Vec3i::new(
        read_u64(&buffer, index     ) as usize,
        read_u64(&buffer, index +  8) as usize,
        read_u64(&buffer, index + 16) as usize
    )
}