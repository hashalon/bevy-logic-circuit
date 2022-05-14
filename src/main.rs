use bevy::prelude::*;


mod circuit;


// enforce that resets are run before computation
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum Label {
    Reset,
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(circuit::sys_reset_wires .label(Label::Reset))
        .add_system(circuit::sys_tick_muxes  .after(Label::Reset))
        .add_system(circuit::sys_tick_demuxes.after(Label::Reset))
        .add_system(circuit::sys_tick_gates  .after(Label::Reset))
        .run();
}
