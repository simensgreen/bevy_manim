use bevy::
{
    asset::{Handle, Assets},
    app::{Plugin, AppBuilder},
    render::
    {
        mesh::Mesh,
        entity::MeshBundle,
        pipeline::{RenderPipelines, PipelineDescriptor, RenderPipeline},
        shader::{Shader, ShaderStages, ShaderStage}
    },
    ecs::
    {
        entity::Entity,
        system::{IntoSystem, Commands, ResMut, Query, Res}
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

pub struct ManimMesh
{
    mesh_handle: Handle<Mesh>
}

pub struct ManimRenderPipelines(RenderPipelines);


pub struct ManimPlugin;
impl Plugin for ManimPlugin
{
    fn build(&self, app: &mut AppBuilder)
    {
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
        world.insert_resource(ManimRenderPipelines(render_pipelines))
    }
}

fn animation_processor
(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut animated: Query<(Entity, &ManimMesh, &mut ScheduledAnimation)>,
    time: Res<Time>,
)
{
    let since_start = time.seconds_since_startup() as f32;
    for (entity, drawing_mesh, mut scheduled_animation) in animated.iter_mut()
    {
        if let Some(mesh) = meshes.get_mut(drawing_mesh.mesh_handle.clone())
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
    tmp.set_progress(progress);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, tmp.get_vertices());
    mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, tmp.get_colors());
    mesh.set_indices(Some(tmp.get_indices()));
}

pub fn draw<D: Drawing>
(
    drawing: D,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    pipeline: &Res<ManimRenderPipelines>
) -> Entity
{
    let mesh = drawing.get_mesh();
    let mesh_handle = meshes.add(mesh);
    commands.spawn_bundle
    (
        MeshBundle
        {
            mesh: mesh_handle.clone(),
            render_pipelines: pipeline.0.clone(),
            ..Default::default()
        }
    ).insert(ManimMesh { mesh_handle } ).id()
}

pub fn animate_draw<D: Drawing, A: AnimationKind>(
    drawing: D,
    animation: A,
    start: f32,
    duration: Duration,
    is_loop: bool,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    pipeline: &Res<ManimRenderPipelines>,
) -> Entity
{
    let mesh = drawing.get_mesh();
    let mesh_handle = meshes.add(mesh);
    let animation = drawing.animate(animation);
    commands.spawn_bundle(MeshBundle
    {
        mesh: mesh_handle.clone(),
        render_pipelines: pipeline.0.clone(),
        ..Default::default()
    })
        .insert(ManimMesh { mesh_handle })
        .insert(ScheduledAnimation
        {
            animation, is_loop, start, duration
        })
        .id()
}