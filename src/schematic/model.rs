/**
 * represent a model to load, build and to display in bevy
 */
use crate::circuit::BundleModel;
use crate::math::*;
use crate::schematic::Index;
use bevy::{prelude::*, render::mesh::*};
use serde::{Deserialize, Serialize};


// indicate position of the model and model to use
#[derive(Serialize, Deserialize)]
pub struct ModelAttr {
    pub position : Vec3i,
    pub index    : Index,
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
        let mut vertexes = Vec::<Vertex>::with_capacity(size * PER_BOX_VERTEXES);
        let mut indexes  = Vec::<u32   >::with_capacity(size * PER_BOX_INDEXES );

        // add vertices and indices to the lists
        for abox in self.0.iter() {
            add_to_model(&abox, &mut vertexes, &mut indexes);
        }

        let indices  = Indices::U32(indexes);
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertexes);
        return mesh;
    }
}


const PER_BOX_VERTEXES : usize = 8; // count vertexes
const PER_BOX_INDEXES  : usize = 6 * 6; // count indexes

// template
const VERTEXES: [Vec3i; PER_BOX_VERTEXES] = [
    Vec3i::new(0, 0, 0), // front-bottom-left
    Vec3i::new(1, 0, 0), // front-bottom-right
    Vec3i::new(1, 1, 0), // front-top-right
    Vec3i::new(0, 1, 0), // front-top-left
    Vec3i::new(0, 0, 1), // back-bottom-left
    Vec3i::new(1, 0, 1), // back-bottom-right
    Vec3i::new(1, 1, 1), // back-top-right
    Vec3i::new(0, 1, 1), // back-top-left
];

// template
const INDEXES: [u32; PER_BOX_INDEXES] = [
    0, 3, 1, 1, 3, 2, // front
    5, 6, 4, 4, 6, 7, // back
    4, 7, 0, 0, 7, 3, // left
    1, 2, 5, 5, 2, 7, // right
    0, 4, 1, 1, 4, 5, // bottom
    3, 7, 2, 2, 7, 6, // top
];


fn add_to_model(abox: &Box3i, vertexes: &mut Vec<Vertex>, indexes: &mut Vec<u32>) {
    // keep track of the new start index
    // before adding new vertexes
    let start = vertexes.len() as u32;
    for i in 0..PER_BOX_INDEXES {
        indexes.push(start + INDEXES[i]);
    }

    // add a point of the box to the model
    for i in 0..PER_BOX_VERTEXES {
        let vert = abox.begin + abox.size() * VERTEXES[i];
        vertexes.push(vert.to_vertex());
    }
}