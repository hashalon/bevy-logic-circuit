use std::{cmp::min, hash::{Hash, Hasher}, collections::HashMap};
use disjoint_hash_set::DisjointHashSet;
use crate::math::{Vec3i, Box3i};
use super::*;


// label type used to analyze morphologic shapes
pub type Label = u32;
pub type FnEmpty<T> = dyn Fn(T) -> bool;


// basic two pass implementation of the 6-connected component labeling algorithm
fn connected_component_labeling<T: Clone + Copy + Eq + Default>
(matrix: &Matrix<T>, is_empty: &FnEmpty<T>) 
-> (Matrix<Label>, HashMap<Label, T>) {

    // prepare map of labels and set of union
    let mut current  = 1;
    let mut labels   = Matrix::<Label>::new(matrix.size, 0);
    let mut disjoint = DisjointHashSet::<Label>::new();
    let mut map_tmp  = HashMap::<Label, T>::with_capacity(matrix.size.index_range() / 6);

    // macro to avoid repeating the same four instructions three times
    macro_rules! associate {
        ($matrix:ident, $set:ident, $i:ident, $a:ident, $b:ident) => {
            {
                let la = $matrix.data[$a];
                let lb = $matrix.data[$b];
                $matrix.data[$i] = min(la, lb);
                $set.link(la, lb);
            }
        };
    }

    /* FIRST PASS */
    // iterate over the whole matrix
    matrix.for_each(&mut |x, y, z| {
        let i = matrix.index(x, y, z);
        let v = matrix.data[i];

        // cells which value is null are simply empty
        if !is_empty(v) {
            // check for combinations using a bitmask
            let mut mask = 0b000usize;

            // compute indexes if necessary
            let mut ix = 0usize;
            let mut iy = 0usize;
            let mut iz = 0usize;

            // if we are not on the first element of each coord,
            // indicate that we need to compare to previous element
            if x > 0 {
                ix = matrix.index(x - 1, y, z);
                if v == matrix.data[ix] {mask |= 0b001;}
            }
            if y > 0 {
                iy = matrix.index(x, y - 1, z);
                if v == matrix.data[iy] {mask |= 0b010;}
            }
            if z > 0 {
                iz = matrix.index(x, y, z - 1);
                if v == matrix.data[iz] {mask |= 0b100;}
            }
            match mask {
                0b111 => {
                    let lx = labels.data[ix];
                    let ly = labels.data[iy];
                    let lz = labels.data[iz];
                    labels.data[i] = min(lx, min(ly, lz));
                    disjoint.link(lx, ly);
                    disjoint.link(lx, lz);
                },
                0b110 => associate!(labels, disjoint, i, iy, iz),
                0b101 => associate!(labels, disjoint, i, ix, iz),
                0b011 => associate!(labels, disjoint, i, ix, iy),
                0b100 => labels.data[i] = labels.data[iz],
                0b010 => labels.data[i] = labels.data[iy],
                0b001 => labels.data[i] = labels.data[ix],
                0b000 => {
                    labels.data[i] = current;
                    disjoint.insert(current);
                    map_tmp .insert(current, v);
                    current += 1;
                },
                _ => {}
            }
        }
    });

    /* SECOND PASS */
    // convert the disjoint-set into a hashmap
    // to join labels into a single one
    let mut replace = HashMap::<Label, Label>::with_capacity(current as usize);
    let mut label: Label = 1;
    for set in disjoint.sets() {
        for elem in set {replace.insert(elem, label);}
        label += 1;
    }
    // simply replace each label by the new jointed one
    for cell in labels.data.iter_mut() {*cell = replace[cell];}

    // also remap labels to corresponding value
    let mut mapping = HashMap::<Label, T>::with_capacity(label as usize);
    for (from, to) in replace {mapping.insert(to, map_tmp[&from]);}

    // return the matrix of labels and the mapping
    // of each label to its corresponding value
    return (labels, mapping);
}

