use bevy::prelude::*;
use super::*;


/* Entity Demultiplexer: CompDemux, PinsIn, PinsOut */
#[derive(Component)]
pub struct CompDemux(pub Data);



pub fn sys_tick(
    comp_query: Query<(&CompDemux, &PinsIn, &PinsOut)>,
    prev_query: Query<&DataPrev>,
    mut next_query: Query<(&PinChannel, &mut DataNext)>
) {
    for (output, pins_in, pins_out) in comp_query.iter() {
 
        // find the values of input wires
        let mut data: Data = 0;
        for id in pins_in.0.iter() {
            if let Ok(pin) = prev_query.get(*id) {
                data |= pin.0;
            }
        }
 
        // apply the value to all output wires
        for id in pins_out.0.iter() {
            if let Ok((index, mut pin)) = next_query.get_mut(*id) {
                if ((data >> index.0) & 1) != 1 {
                    pin.0 |= output.0;
                }
            }
        }
    }
}
 
 