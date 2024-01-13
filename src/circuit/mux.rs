use super::*;

/* Multiplexer Entity: CompMux, PinsIn, PinsOut */
#[derive(Component)]
pub struct CompMux;

// combine multiple input values as boolean into a single wire
pub fn sys_tick(
    comp_query: Query<(&PinsIn, &PinsOut), With<CompMux>>,
    prev_query: Query<(&PinChannel, &DataPrev)>,
    mut next_query: Query<&mut DataNext>,
) {
    for (pins_in, pins_out) in comp_query.iter() {
        // find the values of input wires
        let mut data: Data = 0;
        for id in pins_in.0.iter() {
            if let Ok((index, pin)) = prev_query.get(*id) {
                data |= if pin.0 != 0 { 1 } else { 0 } << index.0;
            }
        }

        // apply the value to all output wires
        for id in pins_out.0.iter() {
            if let Ok(mut pin) = next_query.get_mut(*id) {
                pin.0 |= data;
            }
        }
    }
}
