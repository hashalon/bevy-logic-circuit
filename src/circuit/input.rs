use super::*;
use bevy::input::{keyboard, ButtonState};

/* Keyboard Input Device */
#[derive(Default, Resource)]
pub struct InputDevice {
    buffer: [Data; NB_CHANNELS],
}

/* Keyboard Input Entity: CompInput, PinsOut */
#[derive(Component)]
pub struct CompInput;

// apply computed buffer to output pins
pub fn sys_tock(mut events: EventReader<keyboard::KeyboardInput>, mut device: ResMut<InputDevice>) {
    for event in events.read() {
        // find which word and which bit of the buffer to write to
        let code = event.scan_code as usize;
        let word = code / NB_CHANNELS;
        let bit = code % DATA_SIZE;

        // add or remove a bit from the buffer
        match event.state {
            ButtonState::Pressed => device.buffer[word] |= 1 << bit,
            ButtonState::Released => device.buffer[word] &= !(1 << bit),
        }
    }
}

// apply computed buffer to output pins
pub fn sys_tick(
    device: Res<InputDevice>,
    comp_query: Query<&PinsOut, With<CompInput>>,
    mut next_query: Query<(&PinChannel, &mut DataNext)>,
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
