use crate::core::{Color, Indices};

use crate::draw::{Drawing, Tessellation};
use lyon::algorithms::path::Path;
use lyon::math::Point;
use lyon::tessellation::{LineJoin, StrokeOptions, StrokeTessellator, VertexBuffers, BuffersBuilder, StrokeVertex, FillTessellator, FillOptions, FillVertex};


pub struct Rect
{
    pub left_up: [f32; 2],
    pub right_down: [f32; 2],
    pub width: f32,
    pub joints: LineJoin,
    pub color: Color,
    pub outline: Color,
}

impl Default for Rect
{
    fn default() -> Self
    {
        Self
        {
            left_up: [-0.1, -0.1],
            right_down: [0.1, 0.1],
            width: 0.01,
            joints: LineJoin::Miter,
            color: [1.0, 1.0, 1.0, 0.0],
            outline: [1.0, 1.0, 1.0, 1.0]
        }
    }
}

impl Drawing for Rect
{
    fn get_tessellation(&self) -> Tessellation
    {
        let mut path_builder = Path::builder();
        path_builder.begin(Point::from(self.left_up));
        path_builder.line_to(Point::new(self.left_up[0], self.right_down[1]));
        path_builder.line_to(Point::from(self.right_down));
        path_builder.line_to(Point::new(self.right_down[0], self.left_up[1]));
        path_builder.end(true);
        let path = path_builder.build();
        let mut buffer: VertexBuffers<[f32; 3], u16> = VertexBuffers::new();
        let mut tesselator = FillTessellator::new();
        tesselator.tessellate_path(
            &path,
            &FillOptions::default(),
            &mut BuffersBuilder::new
                (
                    &mut buffer,
                    |vertex: FillVertex| { [vertex.position().x, vertex.position().y, 0.0_f32] }
                )
        ).unwrap();
        let mut colors= vec![self.color; buffer.vertices.len()];
        if self.width > 0.0
        {
            let mut options = StrokeOptions::default();
            options.line_width = self.width;
            options.tolerance = self.width / 90.0;
            options.line_join = self.joints;
            let mut tessellator = StrokeTessellator::new();
            tessellator.tessellate_path
            (
                &path,
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
            colors
        }
    }
}

