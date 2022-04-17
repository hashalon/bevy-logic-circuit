use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(setup_physics.system())
        .add_system(print_ball_altitude.system())
        .run();
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(100.0, 0.1, 100.0).into(),
        ..Default::default()
    };
    commands.spawn_bundle(collider);

    /* Create the bouncing ball. */
    let rigid_body = RigidBodyBundle {
        position: Vec3::new(0.0, 10.0, 0.0).into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::ball(0.5).into(),
        material: ColliderMaterial {
            restitution: 0.7,
            ..Default::default()
        }.into(),
        ..Default::default()
    };
    commands.spawn_bundle(rigid_body)
        .insert_bundle(collider);
}

fn print_ball_altitude(positions: Query<&RigidBodyPositionComponent>) {
    for rb_pos in positions.iter() {
        println!("Ball altitude: {}", rb_pos.position.translation.vector.y);
    }
}