/**
 * represent a model to build and to display in bevy
 */
use crate::math::vec3i::*;


#[derive(Clone, Copy)]
pub struct Box3i {
    pub point: Vec3i,
    pub size:  Vec3i,
}


pub const PER_BOX_VERTEXES: usize = 8; // count vertexes
pub const PER_BOX_INDEXES:  usize = 6 * 6; // count indexes


// template
const VERTEXES: [Vec3i; PER_BOX_VERTEXES] = [
    Vec3i{x: 0, y: 0, z: 0}, // front-bottom-left
    Vec3i{x: 1, y: 0, z: 0}, // front-bottom-right
    Vec3i{x: 1, y: 1, z: 0}, // front-top-right
    Vec3i{x: 0, y: 1, z: 0}, // front-top-left
    Vec3i{x: 0, y: 0, z: 1}, // back-bottom-left
    Vec3i{x: 1, y: 0, z: 1}, // back-bottom-right
    Vec3i{x: 1, y: 1, z: 1}, // back-top-right
    Vec3i{x: 0, y: 1, z: 1}, // back-top-left
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


impl Box3i {
    pub fn add_to_model(&self,
        vertexes: &mut Vec<Vertex>,
        indexes:  &mut Vec<u32>
    ) {
        // keep track of the new start index
        // before adding new vertexes
        let start = vertexes.len() as u32;
        for i in 0..PER_BOX_INDEXES {
            indexes.push(start + INDEXES[i]);
        }

        // add a point of the box to the model
        for i in 0..PER_BOX_VERTEXES {
            vertexes.push((self.point + self.size * VERTEXES[i]).to_vertex());
        }
    }
}