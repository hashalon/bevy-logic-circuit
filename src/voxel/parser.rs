use std::{cmp::min, hash::{Hash, Hasher}, collections::HashMap};
use disjoint_hash_set::DisjointHashSet;
use petgraph::csr::Csr;
use bit_vec::BitVec;
use fasthash::MetroHasher;
use crate::math::{Vec3i, Box3i};
use crate::schematic::{Schema, ModelData};
use crate::voxel::Matrix;


// label type used to analyze morphologic shapes
pub type Label     = u32;
pub type Signature = u64;


// regroup the type of the component, its position and the model to use
#[derive(Clone)]
pub struct ComponentData<T: Clone> {
    label    : Label, 
    value    : T,
    position : Vec3i,
    volume   : usize,
    signature: Signature,
}

impl<T: Clone> ComponentData<T> {
    // make a new container of component data
    pub fn new(label: Label, value: T, position: Vec3i, volume: usize, signature: Signature) -> Self {
        Self {
            label    : label,
            value    : value,
            position : position,
            volume   : volume,
            signature: signature,
        }
    }
}


impl<T: Clone + Copy + Default> ComponentData<T> {
    fn default() -> Self {
        Self {
            label    : 0,
            value    : T::default(),
            position : Vec3i::new(0, 0, 0),
            volume   : 0,
            signature: 0,
        }
    }
}


// parse the matrix and deduce data that will be used to make a schematic
pub fn parse_matrix<T: Clone + Copy + Eq + Default>(matrix: &Matrix<T>, threshold: usize) 
-> (Csr<Label, ()>, Vec<ComponentData<T>>, HashMap<Signature, ModelData>) {
    
    // generate a matrix with a label for each component
    let (labels_matrix, labels_mapping) = connected_component_labeling(matrix);
    let labels_amount = labels_mapping.len();

    // find how components are connected togethers
    let graph = find_connections(&labels_matrix, labels_amount, threshold);

    // find the minimal bounding box of each component
    let boxes = find_bounding_boxes(&labels_matrix, labels_amount);

    // build two lists with component data and model
    let mut components = Vec::<ComponentData<T>>::with_capacity(labels_amount);
    components.resize(labels_amount, ComponentData::default());
    let mut models = HashMap::<Signature, ModelData>::with_capacity(labels_amount);

    // for each label, generate corresponding component data
    for (index, abox) in boxes.iter().enumerate() {
        let label = (index + 1) as Label;
        let value = labels_mapping[&label];

        // find the morphological signature
        let (signature, volume) = generate_signature(&labels_matrix, label, *abox);
        components[index] = ComponentData::<T>::new(label, value, abox.begin, volume, signature);

        // if the component has a new morphology, generate a model for it
        if !models.contains_key(&signature) {
            models.insert(signature, generate_model(&labels_matrix, label, *abox));
        }
    }
    models.shrink_to_fit();
    return (graph, components, models);
}


// basic two pass implementation of the 6-connected component labeling algorithm
fn connected_component_labeling<T: Clone + Copy + Eq + Default>(matrix: &Matrix<T>) 
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
    let empty = T::default();
    matrix.for_each(&mut |x, y, z| {
        let i = matrix.index(x, y, z);
        let v = matrix.data[i];

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



// find the bounding for each label in the matrix
fn find_bounding_boxes(matrix: &Matrix<Label>, labels_amount: usize) -> Vec<Box3i> {
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


// generate signatures for a each component
fn generate_signature(matrix: &Matrix<Label>, label: Label, abox: Box3i) -> (Signature, usize) {
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


// analyze the matrix to find connections between components
fn find_connections(matrix: &Matrix<Label>, labels_amount: usize, threshold: usize) -> Csr<Label, ()> {

    // prepare a graph with all the nodes
    let mut graph = Csr::<Label, ()>::new();
    for label in 1..=labels_amount as Label {graph.add_node(label);}

    // traverse the matrix looking for connections
    matrix.for_each(&mut |x, y, z| {
        let label1 = matrix.get(x, y, z);
        if label1 > 0 {

            // find all neighbors
            let neighbors = matrix.get_neighbors(x, y, z, 0);

            // count the number of occurence of each neighbors label
            let mut counters = HashMap::<Label, usize>::with_capacity(6);
            for label2 in neighbors {
                if let Some(count) = counters.get_mut(&label2) {
                    *count += 1;
                } else {
                    counters.insert(label2, 1);
                }
            }

            // find labels which adjacent amount is greater than the threshold
            for (label2, count) in counters {
                if label2 > 0 && label1 != label2 && count >= threshold {
                    graph.add_edge(label1, label2, ());
                }
            }
        }
    });

    return graph;
}