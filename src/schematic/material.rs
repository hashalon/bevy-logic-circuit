/**
 * store the various materials to use for rendering
 */
use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct MaterialStore {
    store: Vec<Handle<StandardMaterial>>,
}

impl MaterialStore {
    pub fn add_material(
        &mut self,
        mut materials: ResMut<Assets<StandardMaterial>>,
        new_material: StandardMaterial,
    ) {
        self.store.push(materials.add(new_material));
    }
}
