use bevy::{
    prelude::{Transform, Vec3},
    render::mesh::{Mesh, Indices, PrimitiveTopology},
    math::f32::{Quat, Mat3},
};
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use crate::math::Vec3i;


#[derive(Serialize, Deserialize)]
pub struct Model {
    indexes   : Vec<u32>,
    positions : Vec<[f32; 3]>,
    normals   : Vec<[f32; 3]>,
}
impl Model {
    pub fn new(
        indexes   : Vec<u32>, 
        positions : Vec<[f32; 3]>, 
        normals   : Vec<[f32; 3]>
    ) -> Self {
        Self {indexes, positions, normals}
    }

    pub fn to_mesh(&self) -> Mesh {
        let uvs = vec![[0.0; 2]; self.positions.len()];
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.positions.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL  , self.normals  .clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0    , uvs);
        mesh.set_indices(Some(Indices::U32(self.indexes.clone())));
        mesh
    }
}


// indicate which model to use and how to use it
#[derive(Serialize, Deserialize)]
pub struct Attribute {
    pub index       : u32,
    pub position    : Vec3i,
    pub orientation : u8,
}

impl Attribute {
    pub fn to_transform(&self) -> Transform {
        Transform { 
            translation: self.position.as_vec3(), 
            rotation: QUATERNIONS[self.orientation as usize], 
            scale: Vec3::ONE,
        }
    }
}

const ORIENT_SIZE: usize = 24;
const ORIENTATIONS: [[i8; 9]; ORIENT_SIZE] = [
    //--- x ---   --- y ---   --- z ---
    [ 1,  0,  0,  0,  1,  0,  0,  0,  1], //  0
    [-1,  0,  0,  0, -1,  0,  0,  0,  1], //  1
    [ 0, -1,  0,  1,  0,  0,  0,  0,  1], //  2
    [ 0,  1,  0, -1,  0,  0,  0,  0,  1], //  3
    [-1,  0,  0,  0,  1,  0,  0,  0, -1], //  4
    [ 1,  0,  0,  0, -1,  0,  0,  0, -1], //  5
    [ 0,  1,  0,  1,  0,  0,  0,  0, -1], //  6
    [ 0, -1,  0, -1,  0,  0,  0,  0, -1], //  7
    [-1,  0,  0,  0,  0,  1,  0,  1,  0], //  8
    [ 1,  0,  0,  0,  0, -1,  0,  1,  0], //  9
    [ 0,  0,  1,  1,  0,  0,  0,  1,  0], // 10
    [ 0,  0, -1, -1,  0,  0,  0,  1,  0], // 11
    [ 1,  0,  0,  0,  0,  1,  0, -1,  0], // 12
    [-1,  0,  0,  0,  0, -1,  0, -1,  0], // 13
    [ 0,  0, -1,  1,  0,  0,  0, -1,  0], // 14
    [ 0,  0,  1, -1,  0,  0,  0, -1,  0], // 15
    [ 0,  0, -1,  0,  1,  0,  1,  0,  0], // 16
    [ 0,  0,  1,  0, -1,  0,  1,  0,  0], // 17
    [ 0,  1,  0,  0,  0,  1,  1,  0,  0], // 18
    [ 0, -1,  0,  0,  0, -1,  1,  0,  0], // 19
    [ 0,  0,  1,  0,  1,  0, -1,  0,  0], // 20
    [ 0,  0, -1,  0, -1,  0, -1,  0,  0], // 21
    [ 0,  1,  0,  0,  0, -1, -1,  0,  0], // 22
    [ 0, -1,  0,  0,  0,  1, -1,  0,  0], // 23
];

const QUATERNIONS: Lazy<[Quat; ORIENT_SIZE]> = 
Lazy::new(|| ORIENTATIONS.map(|a| to_quat(&a)));


fn to_quat(row: &[i8; 9]) -> Quat {
    let flt = row.map(|n| n as f32);
    let mat = Mat3::from_cols_array(&flt);
    Quat::from_mat3(&mat)
}