use std::hash::{Hash, Hasher};
use bit_vec::BitVec;
use fasthash::MetroHasher;
use crate::math::{Vec3i, Box3i};
use crate::schematic::ModelData;
use super::*;


// label type used to analyze morphologic shapes
pub type Morph = u64;


// find the bounding for each label in the matrix
pub fn find_bounding_boxes(matrix: &Matrix<Label>, labels_amount: usize) -> Vec<Box3i> {
    // define a box for each label
    let mut boxes = Vec::<Box3i>::with_capacity(labels_amount);
    boxes.resize(labels_amount, Box3i::new(matrix.size, Vec3i::new(0, 0, 0)));

    // find the smallest bounding box for each component of the matrix
    matrix.for_each(&mut |x, y, z| {
        let label = matrix.get(x, y, z);
        if label > 0 {
            let index = (label - 1) as usize;
            let curr  = Vec3i::new(x, y, z);
            let abox  = boxes[index];
            boxes[index] = Box3i::new(abox.begin.min(curr), abox.end.max(curr));
        }
    });
    return boxes;
}


// generate morphological signatures for a each component
pub fn generate_morph(matrix: &Matrix<Label>, label: Label, abox: Box3i) -> (Morph, usize) {
    // prepare a bitvec to represent the morphological pattern
    let mut bitvec = BitVec::from_elem(abox.size().index_range(), false);

    // analyze the portion of the matrix to deduce a morphologic signature for the label
    let mut index = 0usize;
    let mut count = 0usize;
    matrix.for_each_in_box(abox, &mut |x, y, z| {
        if label == matrix.get(x, y, z) {
            bitvec.set(index, true);
            count += 1;
        }
        index += 1;
    });
    
    // generate the signature
    let mut hasher = MetroHasher::default();
    abox.size().hash(&mut hasher);
    bitvec.hash(&mut hasher);

    // return the signature and the number of cells covered by the shape
    return (hasher.finish(), count);
}


// TODO: remove this part and replace by simple matrix to be used with "block-mesh"
// generate a box model for the component
#[deprecated]
fn generate_model(matrix: &Matrix<Label>, label: Label, abox: Box3i) -> ModelData {
    let mut boxes = Vec::<Box3i>::with_capacity(abox.size().sum());

    // build boxes until all cells of the component are covered
    matrix.for_each_in_box(abox, &mut |x, y, z| {
        if label == matrix.get(x, y, z) {
            let begin = Vec3i::new(x, y, z);

            // generate a new box if the cell is not already part of an other box
            let mut to_add = true;
            for bbox in boxes.iter() {
                if bbox.bounds(begin) {
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
#[deprecated]
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