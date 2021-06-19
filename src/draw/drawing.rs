use crate::core::{Indices, Color, Vertex, PackedAnimation, AnimationKind};
use bevy::render::mesh::Mesh;
use bevy::render::pipeline::PrimitiveTopology;

#[derive(Debug, Clone)]
pub struct Tessellation
{
    pub vertices: Vec<Vertex>,
    pub indices: Indices,
    pub colors: Vec<Color>,
}

impl Default for Tessellation
{
    fn default() -> Self
    {
        Self
        {
            vertices: vec![],
            indices: Indices::U16(vec![]),
            colors: vec![]
        }
    }
}

pub trait Drawing
{
    fn get_tessellation(&self) -> Tessellation;
    fn get_mesh(&self) -> Mesh
    {
        let tessellation = self.get_tessellation();
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, tessellation.vertices);
        mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, tessellation.colors);
        mesh.set_indices(Some(tessellation.indices));
        mesh
    }
    fn animate<A: AnimationKind>(&self, animation: A) -> PackedAnimation
    { animation.generate(self.get_tessellation()) }
}