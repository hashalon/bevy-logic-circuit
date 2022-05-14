/**
 * build wire entity
 */
use bevy::prelude::*;
use crate::circuit::base::*;


// wire entity
#[derive(Bundle)]
pub struct WireBundle {
    pub index: PinChannel,
    pub prev: DataPrevious,
    pub next: DataNext,
}

// reset the state of every wire
pub fn sys_reset(
    mut query: Query<(&mut DataPrevious, &mut DataNext)>
) {
    query.for_each_mut(|(mut wire_prev, mut wire_next)| {
        wire_prev.0 = wire_next.0;
        wire_next.0 = 0;
    });
}
