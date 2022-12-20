/**
 * simple component that output a value
 */
use bevy::prelude::*;
use crate::circuit::base::*;


// constant input value
#[derive(Component)]
pub struct Fixed(pub Data);


// constant entity
#[derive(Bundle)]
pub struct FixedBundle {
    pub comp: Fixed,
    pub pins_out: PinsOut,
}


// simply apply the constant
pub fn sys_tick(
    comp_query: Query<(&Fixed, &PinsOut)>,
    mut next_query: Query<&mut DataNext>
) {
    for (constant, pins_out) in comp_query.iter() {

        // apply the value to all output wires
        for id in pins_out.0.iter() {
            if let Ok(mut pin) = next_query.get_mut(*id) {
                pin.0 |= constant.0;
            }
        }
    }
}
