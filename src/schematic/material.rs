/**
 * store the various materials to use for rendering
 */
use bevy::prelude::*;


#[derive(Resource)]
pub struct MaterialStore {
    store : Vec<Handle<StandardMaterial>>,
}


impl Default for MaterialStore {
    fn default() -> Self {
        Self {
            store: Vec::default(),
        }
    }
}


impl MaterialStore {
    pub fn add_material (
        &mut self,
        mut materials : ResMut<Assets<StandardMaterial>>,
        new_material  : StandardMaterial
    ) {
        self.store.push(materials.add(new_material));
    }
}
