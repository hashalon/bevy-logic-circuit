//! Create a custom material to draw basic lines in 3D

use bevy::{
    prelude::*,
    render::render_resource::*,
};

mod math;
mod circuit;
mod schematic;

use circuit::*;
use schematic::*;


fn main() {

    let schema = Schema::default();


    App::new()
    
    // default plugins to display window and setup renderer
    .add_plugins(DefaultPlugins)

    // construct the circuitry from schematic
    .insert_resource(schema)
    .add_startup_system(build_circuit)
    //.add_startup_system(start_test)
    
    // add the systems that will run the circuitry
    .add_plugin(CircuitPlugin)
    .run();
}


fn start_test (
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let shape = meshes.add(shape::Cube::default().into());
    let mat = materials.add(StandardMaterial {
        base_color: Color::AZURE,
        ..default()
    });


    cmd.spawn(
        PbrBundle {
            mesh: shape,
            material: mat,
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..default()
        }
    );

    cmd.spawn(
        PointLightBundle {
            point_light: PointLight {
                intensity: 9000.0,
                range: 100.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(8.0, 16.0, 8.0),
            ..default()
        }
    );

    cmd.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6.0, 12.0)
        .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        ..default()
    });
}


fn hello () {
    println!("Hello !");
}
