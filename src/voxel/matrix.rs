use serde::{Deserialize, Serialize};
use std::{cmp::min, hash::{Hash, Hasher}, collections::HashMap};
use disjoint_hash_set::DisjointHashSet;
use bit_vec::BitVec;
use fasthash::MetroHasher;
use crate::math::{Vec3i, Box3i};
use crate::schematic::ModelData;


// label type used to analyze morphologic shapes
type Label = u32;


// indicate position of the model and model to use
#[derive(Serialize, Deserialize)]
pub struct Matrix<T> {
    pub size : Vec3i,
    pub data : Vec<T>
}


// used by first pass of connected component labeling
macro_rules! __associate {
    ($matrix:ident, $disjoint:ident, $i:ident, $a:ident, $b:ident) => {
        {
            let la = $matrix.data[$a];
            let lb = $matrix.data[$b];
            $matrix.data[$i] = min(la, lb);
            $disjoint.link(la, lb);
        }
    };
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
    const fn index(&self, x: usize, y: usize, z: usize) -> usize {
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

    // basic two pass implementation of the 6-connected component labeling algorithm
    pub fn connected_component_labeling(&self, empty: T) -> (Matrix<Label>, Label) {
        // prepare map of labels and set of union
        let mut current  = 1;
        let mut matrix   = Matrix::<Label>::new(self.size, 0);
        let mut disjoint = DisjointHashSet::<Label>::new();

        /* FIRST PASS */
        // iterate over the whole matrix
        self.for_each(&mut |x, y, z| {
            let i = self.index(x, y, z);
            let v = self.data[i];

            // cells which value is null are simply empty
            if v != empty {
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
                        disjoint.link(lx, ly);
                        disjoint.link(lx, lz);
                    },
                    0b110 => __associate!(matrix, disjoint, i, iy, iz),
                    0b101 => __associate!(matrix, disjoint, i, ix, iz),
                    0b011 => __associate!(matrix, disjoint, i, ix, iy),
                    0b100 => matrix.data[i] = matrix.data[iz],
                    0b010 => matrix.data[i] = matrix.data[iy],
                    0b001 => matrix.data[i] = matrix.data[ix],
                    0b000 => {
                        matrix.data[i] = current;
                        disjoint.insert(current);
                        current += 1;
                    },
                    _ => {}
                }
            }
        });

        /* SECOND PASS */
        // convert the disjoint-set into a hashmap
        // to join labels into a single one
        let mut map = HashMap::<Label, Label>::new();
        let mut label: Label = 1;
        for set in disjoint.sets() {
            for elem in set {map.insert(elem, label);}
            label += 1;
        }
        // simply replace each label by the new jointed one
        for cell in matrix.data.iter_mut() {
            *cell = map[cell];
        }
        return (matrix, label);
    }
}



// find the bounding for each label in the matrix
fn find_bounding_boxes(matrix: &Matrix<Label>, labels_amount: usize) -> Vec<Box3i> {
    // define a box for each label
    let mut boxes = Vec::<Box3i>::with_capacity(labels_amount);
    boxes.resize(labels_amount, Box3i::new(matrix.size, Vec3i::new(0, 0, 0)));

    // find the smallest bounding box for each component of the matrix
    matrix.for_each(&mut |x, y, z| {
        let label = matrix.get(x, y, z) as usize;
        let curr  = Vec3i::new(x, y, z);
        let abox  = boxes[label];
        boxes[label] = Box3i::new(abox.begin.min(curr), abox.end.max(curr));
    });
    return boxes;
}


// generate signatures for a each component
fn generate_signature(matrix: &Matrix<Label>, label: Label, abox: Box3i) -> u64 {
    // prepare a bitvec to represent the morphological pattern
    let mut bitvec = BitVec::from_elem(abox.size().index_range(), false);

    // analyze the portion of the matrix to deduce a morphologic signature for the label
    let mut index = 0usize;
    matrix.for_each_in_box(abox, &mut |x, y, z| {
        if label == matrix.get(x, y, z) {bitvec.set(index, true);}
        index += 1;
    });
    let mut hasher = MetroHasher::default();
    bitvec.hash(&mut hasher);
    hasher.finish()
}


// generate a box model for the component
fn generate_model(matrix: &Matrix<Label>, label: Label, abox: Box3i) -> ModelData {
    let mut boxes = Vec::<Box3i>::with_capacity(abox.size().sum());

    // build boxes until all cells of the component are covered
    matrix.for_each_in_box(abox, &mut |x, y, z| {
        if label == matrix.get(x, y, z) {
            let begin = Vec3i::new(x, y, z);

            // generate a new box if the cell is not already part of an other box
            let mut to_add = true;
            for bbox in boxes.iter() {
                if bbox.inside(begin) {
                    to_add = false;
                    break;
                }
            }
            // the box can be generated an added
            if to_add {
                let end = group_box(matrix, label, begin, abox.end);
                boxes.push(Box3i::new(begin - abox.begin, end - abox.begin));
            }
        }
    });
    boxes.shrink_to_fit();
    return ModelData(boxes);
}


// find the end point of a new box to generate
fn group_box(matrix: &Matrix<Label>, label: Label, from: Vec3i, to: Vec3i) -> Vec3i {
    let mut end_point = to;
    // group a line
    'group_x: for x in (from.x + 1)..to.x {
        if label != matrix.get(x, from.y, from.z) {
            end_point.x = x;
            break 'group_x;
        }
    }
    // group a plane
    'group_y: for y in (from.y + 1)..to.y {
        for x in from.x..to.x {
            if label != matrix.get(x, y, from.z) {
                end_point.y = y;
                break 'group_y;
            }
        }
    }
    // group a volume
    'group_z: for z in (from.z + 1)..to.z {
        for y in from.y..to.y {
            for x in from.x..to.x {
                if label != matrix.get(x, y, z) {
                    end_point.z = z;
                    break 'group_z;
                }
            }
        }
    }
    return end_point;
}