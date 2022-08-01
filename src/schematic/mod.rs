/**
 * Plugin for running logic circuits
 */


mod model;
mod component;
mod schema;


pub use model::{ModelIndex, ModelAttr, ModelData};
pub use component::{CompIndex, CompType, CompData, CompWire};
pub use schema::*;
