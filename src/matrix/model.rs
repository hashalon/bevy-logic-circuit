/**
 * convert the matrix into a mesh of triangles
 */
use bevy::render::{
    mesh::{Indices, VertexAttributeValues},
    options::WgpuOptions,
    render_resource::{PrimitiveTopology, WgpuFeatures},
};
use block_mesh::{
    greedy_quads, visible_block_faces, 
    GreedyQuadsBuffer, MergeVoxel, UnitQuadBuffer, Voxel, VoxelVisibility, 
    RIGHT_HANDED_Y_UP_CONFIG,
};
use super::*;


// https://github.com/bonsairobo/block-mesh-rs/blob/main/examples-crate/render/main.rs


fn generate_mesh() -> Mesh {

}


#[derive(Clone, Copy, Eq, PartialEq)]
struct BoolVoxel(bool);
impl Voxel for BoolVoxel {
    fn get_visibility(&self) -> VoxelVisibility {
        if self.0 {VoxelVisibility::Opaque} else {VoxelVisibility::Empty}
    }
}
impl MergeVoxel for BoolVoxel {
    type MergeValue = Self;
    type MergeValueFacingNeighbour = Self;
    fn merge_value(&self) -> Self::MergeValue {*self}
    fn merge_value_facing_neighbour(&self) -> Self::MergeValueFacingNeighbour {*self}
}
