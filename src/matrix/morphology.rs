use super::*;
use crate::math::{Box3i, Vec3i};
use crate::schematic;
use bit_vec::BitVec;
use block_mesh::ndshape::{ConstShape, ConstShape3u32};
use block_mesh::{
    greedy_quads, GreedyQuadsBuffer, MergeVoxel, Voxel, VoxelVisibility, RIGHT_HANDED_Y_UP_CONFIG,
};
use fxhash::FxHasher;
use std::hash::{Hash, Hasher};

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
            let curr = Vec3i::new(x, y, z);
            let abox = boxes[index];
            boxes[index] = Box3i::new(abox.begin.min(curr), abox.end.max(curr));
        }
    });
    boxes
}

// generate morphological signatures for a each component and find the associated volume
pub fn generate_morph(matrix: &Matrix<Label>, label: Label, abox: Box3i) -> (Morph, usize) {
    // prepare a bitvec to represent the morphological pattern
    let mut bitvec = BitVec::from_elem(abox.size().index_range(), false);

    // analyze the portion of the matrix to deduce a morphologic signature for the label
    let mut index = 0usize;
    let mut volume = 0usize;
    matrix.for_each_in_box(abox, &mut |x, y, z| {
        if label == matrix.get(x, y, z) {
            bitvec.set(index, true);
            volume += 1;
        }
        index += 1;
    });

    // generate the signature
    let mut state = FxHasher::default();
    abox.size().hash(&mut state);
    bitvec.hash(&mut state);

    // return the signature and the volume of the shape
    (state.finish(), volume)
}

// References:
// https://0fps.net/2012/06/30/meshing-in-a-minecraft-game/
// https://github.com/bonsairobo/block-mesh-rs/blob/main/examples-crate/render/main.rs

// we define the size of a chunk as a cube of 256 * 256 * 256
// the total number of voxels in the cube needs to be smaller than a 2^32 anyway
const SIZE: u32 = 0x100;
type Chunk = ConstShape3u32<SIZE, SIZE, SIZE>;

// this function generate a trimesh from a matrix
pub fn generate_model(matrix: &Matrix<Label>, label: Label) -> schematic::Model {
    // prepare buffer of boolean voxels
    // fill it with true if the given label is present
    let mut voxels = [BoolVoxel(false); Chunk::SIZE as usize];
    for i in 0..Chunk::SIZE {
        let [x, y, z] = Chunk::delinearize(i);
        voxels[i as usize] = BoolVoxel(matrix.get(x as usize, y as usize, z as usize) == label);
    }

    // run the algorithm to find exposed faces
    let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;
    let mut buffer = GreedyQuadsBuffer::new(voxels.len());
    greedy_quads(
        &voxels,       // buffer of voxels to analyze
        &Chunk {},     // chunk format
        [0; 3],        // TODO: starting point
        [SIZE - 1; 3], // TODO: end point
        &faces,        // order of vertices on the face
        &mut buffer,   // output buffer
    );

    // prepare buffers to read data generated from the algorithm
    let num_indices = buffer.quads.num_quads() * 6;
    let num_vertices = buffer.quads.num_quads() * 4;
    let mut indexes = Vec::with_capacity(num_indices);
    let mut positions = Vec::with_capacity(num_vertices);
    let mut normals = Vec::with_capacity(num_vertices);

    // fill the buffer with quads data
    let mut index: u32 = 0;
    for (group, face) in buffer.quads.groups.into_iter().zip(faces.into_iter()) {
        for quad in group.into_iter() {
            indexes.extend_from_slice(&face.quad_mesh_indices(index));
            positions.extend_from_slice(&face.quad_mesh_positions(&quad, 1.0));
            normals.extend_from_slice(&face.quad_mesh_normals());
            index += 1;
        }
    }

    // return the data necessary to build the mesh
    schematic::Model {
        indexes,
        positions,
        normals,
    }
}

// generic boolean voxel to be used with the algorithm
#[derive(Clone, Copy, Eq, PartialEq)]
struct BoolVoxel(bool);
impl Voxel for BoolVoxel {
    fn get_visibility(&self) -> VoxelVisibility {
        if self.0 {
            VoxelVisibility::Opaque
        } else {
            VoxelVisibility::Empty
        }
    }
}
impl MergeVoxel for BoolVoxel {
    type MergeValue = Self;
    fn merge_value(&self) -> Self::MergeValue {
        *self
    }
}
