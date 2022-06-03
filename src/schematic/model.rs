/**
 * represent a model to load, build and to display in bevy
 */
use crate::schematic::*;
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
pub struct ModelData (Vec<Box3i>);


impl ModelData {
    pub fn build_model(&self) -> Mesh {
        let size = self.0.len();

        // build arrays to store model information
        let mut vertexes = Vec::<Vertex>::with_capacity(size * PER_BOX_VERTEXES);
        let mut indexes  = Vec::<u32   >::with_capacity(size * PER_BOX_INDEXES );

        // add vertices and indices to the lists
        for b in self.0.iter() {
            b.add_to_model(&mut vertexes, &mut indexes);
        }

        let indices  = Indices::U32(indexes);
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertexes);
        return mesh;
    }
}
