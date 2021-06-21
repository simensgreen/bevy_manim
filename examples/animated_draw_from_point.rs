use bevy::prelude::*;
use bevy_manim::prelude::*;
use std::time::Duration;

pub fn main() {
    App::build()
        .insert_resource(Msaa { samples: 8 })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(ManimPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
)
{
    commands.spawn_bundle(PerspectiveCameraBundle
    {
        transform: Transform::from_xyz(0.0, 0.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    let line_points =
        vec!
        [
            [0.08, 0.25],
            [-0.08, 0.0],
            [0.08, 0.0],
            [-0.08, -0.25],
        ];
    let line = primitives::Line
    { color: [0.95, 0.75, 0.0, 1.0], points: line_points, ..Default::default() } ;
    commands.spawn()
        .insert(line)
        .insert(AnimationDescription
        {
            animation: emergence::FromPoint::default(),
            start: 3.0,
            duration: Duration::from_secs_f32(3.0),
            is_loop: false
        });
}


