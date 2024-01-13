/**
 * represent a model to load, build and to display in bevy
 */
use bevy::{
    prelude::Mesh,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use serde::{Deserialize, Serialize};

// the actual model representation
#[derive(Serialize, Deserialize)]
pub struct Model {
    pub indexes: Vec<u32>,
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
}

impl Model {
    // convert the model into a mesh
    pub fn to_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.positions.to_owned());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals.to_owned());
        mesh.set_indices(Some(Indices::U32(self.indexes.to_owned())));
        mesh
    }
}
