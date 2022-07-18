/**
 * represent a model to load, build and to display in bevy
 */
use crate::circuit::BundleModel;
use crate::math::*;
use crate::schematic::{Index, Vertex, ModelIndex};
use bevy::{prelude::*, render::mesh::*};
use serde::{Deserialize, Serialize};


// indicate position of the model and model to use
#[derive(Serialize, Deserialize)]
pub struct ModelAttr {
    pub position : Vec3i,
    pub index    : ModelIndex,
}

// the actual model representation
#[derive(Serialize, Deserialize)]
pub struct ModelData (pub Vec<Box3i>);


impl ModelAttr {
    pub fn bundle(&self) -> BundleModel {
        BundleModel {

        }
    }
}


impl ModelData {
    pub fn build_model(&self) -> Mesh {
        let size = self.0.len();

        // build arrays to store model information
        let mut indexes  = Vec::<Index >::with_capacity(size * INDEXES .len());
        let mut vertexes = Vec::<Vertex>::with_capacity(size * VERTEXES.len());



        // add vertices and indices to the lists
        for abox in self.0.iter() {
            let occluded = check_occlusions(abox, &self.0);
            add_to_model(&abox, &occluded, &mut indexes, &mut vertexes);
        }

        let indices  = Indices::U32(indexes);
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertexes);
        return mesh;
    }
}


// indicate if the given face is occluded or not
// match the same order as the 'INDEXES' array
type Occluded = [bool; 6];


// specify which face of the box is fully occluded
fn check_occlusions(abox: &Box3i, boxes: &Vec<Box3i>) -> Occluded {
    let mut occluded: Occluded = [false; 6];
    for bbox in boxes {
        // TODO check each face to see if they are occluded
    }
    return occluded;
}


const INDEXES: [[Index; 6]; 6] = [
    [0, 3, 1, 1, 3, 2], // front
    [5, 6, 4, 4, 6, 7], // back
    [4, 7, 0, 0, 7, 3], // left
    [1, 2, 5, 5, 2, 7], // right
    [0, 4, 1, 1, 4, 5], // bottom
    [3, 7, 2, 2, 7, 6], // top
];

const VERTEXES: [Vertex; 8] = [
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
fn add_to_model(abox: &Box3i, occluded: &Occluded, indexes: &mut Vec<Index>, vertexes: &mut Vec<Vertex>) {
    // keep track of the new start index
    // before adding new vertexes
    let start = vertexes.len() as Index;

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
        let vertex: Vertex = [
            size.x as f32 * shift[0] + abox.begin.x as f32,
            size.y as f32 * shift[1] + abox.begin.y as f32,
            size.z as f32 * shift[2] + abox.begin.z as f32,
        ];
        vertexes.push(vertex);
    }
}