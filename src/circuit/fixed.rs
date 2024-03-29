use super::*;

/* Fixed Value Entity: CompFixed, PinsOut */
#[derive(Component)]
pub struct CompFixed(pub Data);

pub fn sys_tick(comp_query: Query<(&CompFixed, &PinsOut)>, mut next_query: Query<&mut DataNext>) {
    for (constant, pins_out) in comp_query.iter() {
        // apply the value to all output wires
        for id in pins_out.0.iter() {
            if let Ok(mut pin) = next_query.get_mut(*id) {
                pin.0 |= constant.0;
            }
        }
    }
}
