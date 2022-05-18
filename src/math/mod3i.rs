/**
 * represent a model to build and to display in bevy
 */
use crate::math::*;
use bevy::{prelude::*, render::mesh::*};


pub struct Mod3i {
    pub boxes: Vec<Box3i>,
}


impl Mod3i {
    pub fn build_model(&self) -> Mesh {
        let size = self.boxes.len();

        // build arrays to store model information
        let mut vertexes = Vec::<Vertex>::with_capacity(size * PER_BOX_VERTEXES);
        let mut indexes  = Vec::<u32>   ::with_capacity(size * PER_BOX_INDEXES);

        // add vertices and indices to the lists
        for b in self.boxes.iter() {
            b.add_to_model(&mut vertexes, &mut indexes);
        }

        let indices = Indices::U32(indexes);
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertexes);
        return mesh;
    }
}