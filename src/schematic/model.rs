/**
 * represent a model to load, build and to display in bevy
 */
use bevy::render::mesh;
use serde::{Deserialize, Serialize};
use crate::math::*;


// define index for vertex
pub type MeshIndex  = u32;
pub type MeshVertex = [f32; 3];

// define index for models
pub type ModelIndex = u32;


// indicate position of the model and model to use
#[derive(Serialize, Deserialize)]
pub struct ModelAttr {
    pub position : Vec3i,
    pub index    : ModelIndex,
}

// the actual model representation
#[derive(Serialize, Deserialize)]
pub struct ModelData (pub Vec<Box3i>);


impl ModelData {
    pub fn build_mesh(&self) -> mesh::Mesh {
        let size = self.0.len();

        // build arrays to store model information
        let mut indexes = Vec::<MeshIndex>::with_capacity(size * INDEXES .len());
        let mut vertexes = Vec::<MeshVertex>::with_capacity(size * VERTEXES.len());

        // add vertices and indices to the lists
        for (i, abox) in self.0.iter().enumerate() {
            let occluded = check_occlusions(abox, i, &self.0);
            add_to_model(&abox, &occluded, &mut indexes, &mut vertexes);
        }

        let indices  = mesh::Indices::U32(indexes);
        let mut amesh = mesh::Mesh::new(mesh::PrimitiveTopology::TriangleList);
        amesh.set_indices(Some(indices));
        amesh.insert_attribute(mesh::Mesh::ATTRIBUTE_POSITION, vertexes);
        amesh
    }
}


// indicate if the given face is occluded or not
// match the same order as the 'INDEXES' array
type Occluded = [bool; 6];


// specify which face of the box is fully occluded
fn check_occlusions(abox: &Box3i, index: usize, boxes: &Vec<Box3i>) -> Occluded {
    let mut occluded: Occluded = [false; 6];

    // compute three other points
    let point_x = Vec3i::new(abox.end  .x, abox.begin.y, abox.begin.z);
    let point_y = Vec3i::new(abox.begin.x, abox.end  .y, abox.begin.z);
    let point_z = Vec3i::new(abox.begin.x, abox.begin.y, abox.end  .z);

    // for each box in the list, check if pair of points are inside
    for (i, bbox) in boxes.iter().enumerate() {
        if index == i {continue;} // skip current box

        // check each point if they are inside of the box
        let ix = bbox.contains(point_x);
        let iy = bbox.contains(point_y);
        let iz = bbox.contains(point_z);
        let ie = bbox.contains(abox.end);

        // use pair of points to deduce if face is occluded by box
        if ix && iy {occluded[0] = true;}
        if iz && ie {occluded[1] = true;}
        if iy && iz {occluded[2] = true;}
        if ix && ie {occluded[3] = true;}
        if ix && iz {occluded[4] = true;}
        if iy && ie {occluded[5] = true;}
    }
    return occluded;
}


const INDEXES: [[MeshIndex; 6]; 6] = [
    [0, 3, 1, 1, 3, 2], // front
    [5, 6, 4, 4, 6, 7], // back
    [4, 7, 0, 0, 7, 3], // left
    [1, 2, 5, 5, 2, 7], // right
    [0, 4, 1, 1, 4, 5], // bottom
    [3, 7, 2, 2, 7, 6], // top
];

const VERTEXES: [MeshVertex; 8] = [
    [0.0, 0.0, 0.0], // front-bottom-left
    [1.0, 0.0, 0.0], // front-bottom-right
    [1.0, 1.0, 0.0], // front-top-right
    [0.0, 1.0, 0.0], // front-top-left
    [0.0, 0.0, 1.0], // back-bottom-left
    [1.0, 0.0, 1.0], // back-bottom-right
    [1.0, 1.0, 1.0], // back-top-right
    [0.0, 1.0, 1.0], // back-top-left
];

// add the box to the mesh model
fn add_to_model(
    abox    : &Box3i, 
    occluded: &Occluded, 
    indexes : &mut Vec<MeshIndex>, 
    vertexes: &mut Vec<MeshVertex>) {
    
    // keep track of the new start index
    // before adding new vertexes
    let start = vertexes.len() as MeshIndex;

    // add indexes if the face is not occluded
    for i in 0..6 {
        if !occluded[i] {
            for index in INDEXES[i] {
                indexes.push(start + index);
            }
        }
    }

    // add a point of the box to the model
    for shift in VERTEXES {
        let size  = abox.size();
        let vertex: MeshVertex = [
            size.x as f32 * shift[0] + abox.begin.x as f32,
            size.y as f32 * shift[1] + abox.begin.y as f32,
            size.z as f32 * shift[2] + abox.begin.z as f32,
        ];
        vertexes.push(vertex);
    }
}