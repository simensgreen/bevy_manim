use bevy::
{
    asset::{Handle, Assets},
    app::{Plugin, AppBuilder},
    render::
    {
        mesh::Mesh,
        entity::MeshBundle,
        pipeline::{RenderPipelines, PipelineDescriptor, RenderPipeline},
        shader::{Shader, ShaderStages, ShaderStage},
    },
    ecs::
    {
        entity::Entity,
        system::{IntoSystem, Commands, ResMut, Query, Res},
        query::{With, Added},
    },
    core::Time,
};

use crate::play::ScheduledAnimation;
use crate::core::{PackedAnimation, AnimationKind};
use crate::draw::Drawing;
use std::time::Duration;

const VERTEX_SHADER: &str = r#"
#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec4 Vertex_Color;
layout(location = 0) out vec4 v_color;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
    v_color = Vertex_Color;
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 450

layout(location = 0) in vec4 v_color;
layout(location = 0) out vec4 o_Target;

void main() {
    o_Target = v_color;
}
"#;

pub struct ManimRenderPipelines
{
    pub pipelines: RenderPipelines
}


pub struct ManimPlugin;
impl Plugin for ManimPlugin
{
    fn build(&self, app: &mut AppBuilder)
    {
        app.add_plugin(registration::RegisterSystems);
        app.add_system(animation_processor.system());

        let world = app.world_mut();
        let mut shaders = world.get_resource_mut::<Assets<Shader>>().unwrap();
        let pipeline = PipelineDescriptor::default_config(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
            fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
        });
        drop(shaders);
        let mut pipelines = world.get_resource_mut::<Assets<PipelineDescriptor>>().unwrap();
        let pipeline_handle = pipelines.add(pipeline);
        let render_pipelines = RenderPipelines::from_pipelines(vec![RenderPipeline::new(
            pipeline_handle,
        )]);
        world.insert_resource(ManimRenderPipelines { pipelines: render_pipelines })
    }
}

pub struct ManimDrawing;

fn animation_processor
(
    mut commands: Commands,
    mut animated: Query<(Entity, &Handle<Mesh>, &mut ScheduledAnimation), With<ManimDrawing>>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
)
{
    let since_start = time.seconds_since_startup() as f32;
    for (entity, mesh_handle, mut scheduled_animation) in animated.iter_mut()
    {
        if let Some(mesh) = meshes.get_mut(mesh_handle.clone())
        {
            let progress = (since_start - scheduled_animation.start) / scheduled_animation.duration.as_secs_f32();
            if progress <= 0.0
            {
                configure_mesh(mesh, scheduled_animation.animation.clone(), 0.0);
            }
            else if progress > 0.0 && progress < 1.0
            {
                configure_mesh(mesh, scheduled_animation.animation.clone(), progress);
            }
            else if progress >= 1.0
            {
                if scheduled_animation.is_loop
                {
                    scheduled_animation.start = since_start
                }
                else
                {
                    commands.entity(entity).remove::<ScheduledAnimation>();
                }
            }
        }
        else
        {
            commands.entity(entity).remove::<ScheduledAnimation>();
        }
    }
}

fn configure_mesh(mesh: &mut Mesh, animation: PackedAnimation, progress: f32)
{
    let mut tmp = animation.lock().unwrap();

    if tmp.get_progress() == progress { return }

    tmp.set_progress(progress);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, tmp.get_vertices());
    mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, tmp.get_colors());
    mesh.set_indices(Some(tmp.get_indices()));
}

pub struct AnimationDescription<A: AnimationKind>
{
    pub animation: A,
    pub start: f32,
    pub duration: Duration,
    pub is_loop: bool,
}

pub(crate) fn register_drawing<T: Drawing>(mut commands: Commands, query: Query<(Entity, &T), Added<T>>, mut meshes: ResMut<Assets<Mesh>>, pipeline: Res<ManimRenderPipelines>)
{
    for (entity, drawing) in query.iter()
    {
        let mesh = drawing.get_mesh();
        let handle = meshes.add(mesh);
        commands.entity(entity)
            .insert(ManimDrawing)
            .insert_bundle(MeshBundle
            {
                mesh: handle,
                render_pipelines: pipeline.pipelines.clone(),
                ..Default::default()
            });
    }
}

pub(crate) fn register_animation<D: Drawing, A: AnimationKind>(mut commands: Commands, query: Query<(Entity, &D, &AnimationDescription<A>), Added<AnimationDescription<A>>>)
{
    for (entity, drawing, animation) in query.iter()
    {
        commands.entity(entity)
            .remove::<AnimationDescription<A>>()
            .insert(ScheduledAnimation
            {
                is_loop: animation.is_loop,
                start: animation.start,
                duration: animation.duration,
                animation: drawing.animate(animation.animation.clone())
            });
    }
}


mod registration
{
    use bevy::app::{Plugin, AppBuilder};
    use bevy::ecs::system::IntoSystem;
    use crate::bevy::register_animation;
    use crate::bevy::register_drawing;
    use crate::draw::primitives::*;
    use crate::anim::emergence::*;

    macro_rules! nested_register {
        ($app:expr; ($($draw:ident),*) $anim:tt) => {
            $(nested_register!(@register $app; $draw $anim);)*
        };
        (@register $app:expr; $draw:ident ($($anim:ident),*)) => {
            $app.add_system(register_drawing::<$draw>.system());
            $($app.add_system(register_animation::<$draw, $anim>.system());)*
        };
    }

    pub(crate) struct RegisterSystems;
    impl Plugin for RegisterSystems
    {
        fn build(&self, app: &mut AppBuilder)
        {
            nested_register!(
            app;
            (Line, Circle)
            (Fade, FromPoint)
            );
        }
    }
}
