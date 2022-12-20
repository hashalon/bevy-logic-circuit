/**
 * logic components to build circuitry
*/

use bevy::prelude::*;
use bevy::input::{keyboard, ButtonState};
use crate::circuit::base::*;


// pool keyboard inputs and convert it into a data buffer
#[derive(Resource)]
pub struct DataBuffer {
    buffer: [Data; NB_CHANNELS],
}

// build a default buffer for keyboard inputs
impl Default for DataBuffer {
    fn default() -> Self {
        Self { buffer: Default::default() }
    }
}

// allow to input keyboard inputs into the circuit
#[derive(Component)]
pub struct Connector;


// constant entity
#[derive(Bundle)]
pub struct InputBundle {
    pub comp: Connector,
    pub pins_out: PinsOut,
}


// apply computed buffer to output pins
pub fn sys_reset(
    mut events: EventReader<keyboard::KeyboardInput>,
    mut device: ResMut<DataBuffer>,
) {
    for event in events.iter() {

        // find which word and which bit of the buffer to write to
        let code = event.scan_code as usize;
        let word = code / NB_CHANNELS;
        let bit  = code % DATA_SIZE;

        // add or remove a bit from the buffer
        match event.state {
            ButtonState::Pressed  => device.buffer[word] |= 1 << bit,
            ButtonState::Released => device.buffer[word] &= !(1 << bit),
        }
    }
}


// apply computed buffer to output pins
pub fn sys_tick(
    device: Res<DataBuffer>,
    comp_query: Query<&PinsOut, With<Connector>>,
    mut next_query: Query<(&PinChannel, &mut DataNext)>
) {
    for pins_out in comp_query.iter() {


      // apply the data to each output pins based on their index
       for id in pins_out.0.iter() {
            if let Ok((index, mut pin)) = next_query.get_mut(*id) {

                pin.0 |= device.buffer[index.0 as usize];
            }
        }
    }
}