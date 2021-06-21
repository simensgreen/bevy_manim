use bevy::prelude::*;
use bevy_manim::prelude::*;

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
    let points =
        vec!
        [
            [0.08, 0.25],
            [-0.08, 0.0],
            [0.08, 0.0],
            [-0.08, -0.25],
        ];
    commands.spawn().insert(primitives::Polygon
    {
        points,
        color: [0.95, 0.75, 0.0, 1.0],
        outline: [1.0, 1.0, 0.2, 1.0],
        ..Default::default()
    });
}


