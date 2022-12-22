/**
 * Plugin for running logic circuits
 */
use bevy::prelude::*;

mod base;
mod model;
mod schema;


pub use base::*;
pub use model::{ModelIndex, ModelAttr, ModelData};
pub use schema::*;



fn start_build_from_schema(
    mut commands: Commands,
    schema: Res<Schema>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    let mut shapes = Vec::<Handle<Mesh>>::with_capacity(schema.models.len());
    for model in schema.models.iter() {
        shapes.push(meshes.add(model.build_mesh()));
    }


}