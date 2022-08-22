use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_system(rotate)
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-30.0, 30.0, 100.0)
            .looking_at(Vec3::new(0.0, 10.0, 0.0), Vec3::Y),
        ..Default::default()
    });
}

pub fn setup_physics(mut commands: Commands) {
    /*
     * Ground
     */
    let ground_size = 200.1;
    let ground_height = 0.1;

    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(
            0.0,
            -ground_height,
            0.0,
        )))
        .insert(Collider::cuboid(ground_size, ground_height, ground_size));

    commands
        .spawn_bundle(TransformBundle::from(Transform::from_rotation(
            Quat::from_rotation_x(0.2),
        )))
        .with_children(|child| {
            child
                .spawn_bundle(TransformBundle::from(Transform::from_xyz(0., 10.0, 0.)))
                .insert(RigidBody::Dynamic)
                .insert(Collider::ball(20.0))
                .insert(Rotate);
        });
}

#[derive(Debug, Clone, Component)]
pub struct Rotate;

pub fn rotate(time: Res<Time>, mut to_rotate: Query<&mut Transform, With<Rotate>>) {
    for mut transform in &mut to_rotate {
        transform.rotation =
            Quat::from_axis_angle(Vec3::Y, time.time_since_startup().as_secs_f32() * 0.01).into();
    }
}
