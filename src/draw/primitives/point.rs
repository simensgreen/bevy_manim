use crate::core::{Color, Indices};
use crate::draw::{Drawing, Tessellation};
use lyon::math;
use lyon::tessellation::{FillOptions, VertexBuffers, BuffersBuilder, FillVertex, FillTessellator};


pub struct Point
{
    pub position: [f32; 2],
    pub color: Color,
    pub width: f32,
}

impl Default for Point
{
    fn default() -> Self
    {
        Self
        {
            position: [0.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
            width: 0.005,
        }
    }
}

impl Drawing for Point
{
    fn get_tessellation(&self) -> Tessellation
    {
        let mut options = FillOptions::default();
        options.tolerance = self.width / 90.0;
        let mut buffer: VertexBuffers<[f32; 3], u16> = VertexBuffers::new();
        let mut tessellator = FillTessellator::new();
        tessellator.tessellate_circle(math::Point::from(self.position), self.width,
            &options,
            &mut BuffersBuilder::new
                (
                    &mut buffer,
                    |vertex: FillVertex| { [vertex.position().x, vertex.position().y, 0.0_f32] }
                )
        ).unwrap();
        buffer.indices.reverse();
        Tessellation
        {
            colors: vec![self.color; buffer.vertices.len()],
            vertices: buffer.vertices,
            indices: Indices::U16(buffer.indices),
        }
    }
}
