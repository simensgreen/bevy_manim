use crate::core::{Color, Indices};
use crate::draw::{Drawing, Tessellation};
use lyon::math::Point;
use lyon::tessellation::{StrokeOptions, StrokeTessellator, VertexBuffers, BuffersBuilder, StrokeVertex, FillTessellator, FillOptions, FillVertex};

pub struct Circle
{
    pub center: [f32; 2],
    pub radius: f32,
    pub color: Color,
    pub outline: Color,
    pub width: f32,
}

impl Default for Circle
{
    fn default() -> Self
    {
        Self
        {
            center: [0.0, 0.0],
            radius: 0.1,
            color: [1.0, 1.0, 1.0, 0.0],
            outline: [1.0, 1.0, 1.0, 1.0],
            width: 0.01,
        }
    }
}

impl Drawing for Circle
{
    fn get_tessellation(&self) -> Tessellation
    {
        let options = FillOptions::default().with_tolerance(self.width / 90.0);
        let mut buffer: VertexBuffers<[f32; 3], u16> = VertexBuffers::new();
        FillTessellator::new().tessellate_circle(
            Point::from(self.center),
            self.radius,
            &options,
            &mut BuffersBuilder::new
                (
                    &mut buffer,
                    |vertex: FillVertex| { [vertex.position().x, vertex.position().y, 0.0_f32] }
                )
        ).unwrap();
        let mut colors = vec![self.color; buffer.vertices.len()];
        if self.width > 0.0
        {
            let mut options = StrokeOptions::default();
            options.line_width = self.width;
            options.tolerance = self.width / 90.0;
            StrokeTessellator::new().tessellate_circle(
                Point::from(self.center),
                self.radius,
                &options,
                &mut BuffersBuilder::new
                    (
                        &mut buffer,
                        |vertex: StrokeVertex| { [vertex.position().x, vertex.position().y, 0.0_f32] }
                    )
            ).unwrap();
        }
        buffer.indices.reverse();
        colors.append(&mut vec![self.outline; buffer.vertices.len() - colors.len()]);
        Tessellation
        {
            vertices: buffer.vertices,
            indices: Indices::U16(buffer.indices),
            colors,
        }
    }
}
