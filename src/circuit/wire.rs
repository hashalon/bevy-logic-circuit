/**
 * build wire entity
 */
use bevy::prelude::*;
use super::*;


// wire entity
#[derive(Bundle)]
pub struct WireBundle {
    pub channel: PinChannel,
    pub prev: DataPrev,
    pub next: DataNext,
}

impl WireBundle {
    pub fn new(channel: Channel) -> Self {
        Self {
            channel: PinChannel(channel),
            prev: DataPrev(0),
            next: DataNext(0)
        }
    }
}


// reset the state of every wire
pub fn sys_reset(
    mut query: Query<(&mut DataPrev, &mut DataNext)>
) {
    query.for_each_mut(|(mut wire_prev, mut wire_next)| {
        wire_prev.0 = wire_next.0;
        wire_next.0 = 0;
    });
}
