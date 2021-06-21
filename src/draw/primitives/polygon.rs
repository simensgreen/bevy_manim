use crate::core::{Color, Indices};
use crate::draw::{Drawing, Tessellation};
use lyon::algorithms::path::Path;
use lyon::math::Point;
use lyon::tessellation::{LineJoin, StrokeOptions, StrokeTessellator, VertexBuffers, BuffersBuilder, StrokeVertex, FillTessellator, FillOptions, FillVertex};


pub struct Polygon
{
    pub points: Vec<[f32; 2]>,
    pub color: Color,
    pub outline: Color,
    pub width: f32,
    pub joints: LineJoin,
}

impl Default for Polygon
{
    fn default() -> Self
    {
        Self
        {
            points: vec![],
            color: [1.0, 1.0, 0.0, 0.0],
            width: 0.01,
            joints: LineJoin::Miter,
            outline: [1.0, 1.0, 1.0, 1.0]
        }
    }
}

impl Drawing for Polygon
{
    fn get_tessellation(&self) -> Tessellation
    {
        if self.points.len() < 3 { panic!("polygon must have > 3 points") }
        let mut path_builder = Path::builder();
        path_builder.begin(Point::from(self.points.first().unwrap().clone()));
        for i in 1..self.points.len()
        {
            path_builder.line_to(Point::from(self.points[i].clone()));
        }
        path_builder.end(true);
        let path = path_builder.build();
        let mut options = StrokeOptions::default();
        options.line_width = self.width;
        options.tolerance = self.width / 90.0;
        options.line_join = self.joints;
        let mut buffer: VertexBuffers<[f32; 3], u16> = VertexBuffers::new();
        let mut tessellator = FillTessellator::new();
        tessellator.tessellate_path
        (
            &path,
            &FillOptions::default(),
            &mut BuffersBuilder::new
                (
                    &mut buffer,
                    |vertex: FillVertex| { [vertex.position().x, vertex.position().y, 0.0_f32] }
                )
        ).unwrap();
        let mut colors = vec![self.color; buffer.vertices.len()];
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
