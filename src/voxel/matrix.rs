use serde::{Deserialize, Serialize};
use crate::math::{Vec3i, Box3i};



// indicate position of the model and model to use
#[derive(Serialize, Deserialize)]
pub struct Matrix<T> {
    pub size : Vec3i,
    pub data : Vec<T>
}


impl<T: Clone + Copy + Eq> Matrix<T> {
    pub fn new(size: Vec3i, value: T) -> Self {
        // allocate a vector with the correct size
        let buffer_size = size.x * size.y * size.z;
        let mut buffer  = Vec::<T>::with_capacity(buffer_size);
        buffer.resize(buffer_size, value);

        Self {
            size: size,
            data: buffer
        }
    }

    #[inline]
    pub const fn index(&self, x: usize, y: usize, z: usize) -> usize {
        x + (y + z * self.size.y) * self.size.x
    }
    
    #[inline]
    pub fn get(&self, x: usize, y: usize, z: usize) -> T {
        self.data[self.index(x, y, z)]
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, z: usize, value: T) {
        let index = self.index(x, y, z);
        self.data[index] = value;
    }

    // apply given function for each cell of the matrix
    pub fn for_each(&self, func: &mut dyn FnMut(usize, usize, usize)) {
        for z in 0..self.size.z {
            for y in 0..self.size.y {
                for x in 0..self.size.x {
                    func(x, y, z);
                }
            }
        }
    }

    // apply given function for each cell in given bounding box
    pub fn for_each_in_box(&self, abox: Box3i, func: &mut dyn FnMut(usize, usize, usize)) {
        for z in abox.begin.z..abox.end.z {
            for y in abox.begin.y..abox.end.y {
                for x in abox.begin.x..abox.end.x {
                    func(x, y, z);
                }
            }
        }
    }
}
