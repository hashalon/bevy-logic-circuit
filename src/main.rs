use bevy::prelude::*;


mod circuit;



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(circuit::sys_tick_wire)
        .add_system(circuit::sys_tick_gate)
        .run();
}
