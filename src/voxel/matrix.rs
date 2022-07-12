use serde::{Deserialize, Serialize};
use partitions::PartitionVec;
use std::{cmp::min, collections::HashMap};
use crate::math::Vec3i;

// indicate position of the model and model to use
#[derive(Serialize, Deserialize)]
pub struct Matrix {
    pub size : Vec3i,
    pub data : Vec<usize>
}


// used by first pass of connected component labeling
macro_rules! __associate {
    ($matrix:ident, $partition:ident, $i:ident, $a:ident, $b:ident) => {
        {
            let la = $matrix.data[$a];
            let lb = $matrix.data[$b];
            $matrix.data[$i] = min(la, lb);
            $partition.union(la, lb);
        }
    };
}


impl Matrix {
    pub fn new(size: Vec3i) -> Self {
        // allocate a vector with the correct size
        let buffer_size = size.x * size.y * size.z;
        let mut buffer  = Vec::<usize>::with_capacity(buffer_size);
        buffer.resize(buffer_size, 0);

        Self {
            size: size,
            data: buffer
        }
    }

    #[inline]
    const fn index(&self, x: usize, y: usize, z: usize) -> usize {
        x + (y + z * self.size.y) * self.size.x
    }
    
    #[inline]
    pub fn get(&self, index: Vec3i) -> usize {
        self.data[self.index(index.x, index.y, index.z)]
    }

    #[inline]
    pub fn set(&mut self, index: Vec3i, value: usize) {
        let index = self.index(index.x, index.y, index.z);
        self.data[index] = value;
    }

    // basic two pass implementation of the 6-connected component labeling algorithm
    pub fn connected_component_labeling(&self) -> Matrix {
        // prepare map of labels and set of union
        let mut current   = 1;
        let mut matrix    = Matrix::new(self.size);
        let mut partition = PartitionVec::<usize>::with_capacity(self.size.index_range() / 6);

        /* FIRST PASS */

        // iterate over the whole matrix
        for z in 0..self.size.z {
            for y in 0..self.size.y {
                for x in 0..self.size.x {

                    let i = self.index(x, y, z);
                    let v = self.data[i];

                    // cells which value is null are simply empty
                    if v > 0 {
                        // check for combinations using a bitmask
                        let mut mask = 0b000usize;

                        // compute indexes if necessary
                        let mut ix = 0usize;
                        let mut iy = 0usize;
                        let mut iz = 0usize;

                        // if we are not on the first element of each coord,
                        // indicate that we need to compare to previous element
                        if x > 0 {
                            ix = self.index(x - 1, y, z);
                            if v == self.data[ix] {mask |= 0b001;}
                        }
                        if y > 0 {
                            iy = self.index(x, y - 1, z);
                            if v == self.data[iy] {mask |= 0b010;}
                        }
                        if z > 0 {
                            iz = self.index(x, y, z - 1);
                            if v == self.data[iz] {mask |= 0b100;}
                        }

                        match mask {
                            0b111 => {
                                let lx = matrix.data[ix];
                                let ly = matrix.data[iy];
                                let lz = matrix.data[iz];
                                matrix.data[i] = min(lx, min(ly, lz));
                                partition.union(lx, ly);
                                partition.union(lx, lz);
                            },
                            0b110 => __associate!(matrix, partition, i, iy, iz),
                            0b101 => __associate!(matrix, partition, i, ix, iz),
                            0b011 => __associate!(matrix, partition, i, ix, iy),
                            0b100 => matrix.data[i] = matrix.data[iz],
                            0b010 => matrix.data[i] = matrix.data[iy],
                            0b001 => matrix.data[i] = matrix.data[ix],
                            0b000 => {
                                matrix.data[i] = current;
                                partition.push(current);
                                current += 1;
                            },
                            _ => {}
                        }
                    }
                }
            }
        }

        /* SECOND PASS */

        // convert the disjoint-set into a hashmap
        // to join labels into a single one
        let mut map = HashMap::<usize, usize>::new();
        for i in 0..partition.amount_of_sets() {
            let set = partition.set(i);
            for (index, value) in set {
                map.insert(*value, index + 1);
            }
        }

        // simply replace each label by the new jointed one
        for cell in matrix.data.iter_mut() {
            *cell = map[cell];
        }

        return matrix;
    }
}

// in case we want to implement iterator for matrices
// https://medium.com/journey-to-rust/iterators-in-rust-a73560f796ee