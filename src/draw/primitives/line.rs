use crate::core::{Color, Indices};
use crate::draw::{Drawing, Tessellation};
use lyon::algorithms::path::Path;
use lyon::math::Point;
use lyon::tessellation::
{
    LineJoin,
    LineCap,
    StrokeOptions,
    StrokeTessellator,
    VertexBuffers,
    BuffersBuilder,
    StrokeVertex
};



pub struct Line
{
    pub points: Vec<[f32; 2]>,
    pub color: Color,
    pub width: f32,
    pub caps: LineCap,
    pub joints: LineJoin
}

impl Default for Line
{
    fn default() -> Self
    {
        Self
        {
            points: vec![],
            color: [1.0, 1.0, 1.0, 1.0],
            width: 0.01,
            caps: LineCap::Round,
            joints: LineJoin::Round
        }
    }
}

impl Drawing for Line
{
    fn get_tessellation(&self) -> Tessellation
    {
        if self.points.len() < 2 { panic!("line must have > 2 points") }
        let mut path_builder = Path::builder();
        path_builder.begin(Point::from(self.points.first().unwrap().clone()));
        for i in 1..self.points.len()
        {
            path_builder.line_to(Point::from(self.points[i].clone()));
        }
        path_builder.end(false);
        let path = path_builder.build();
        let mut options = StrokeOptions::default();
        options.line_width = self.width;
        options.tolerance = self.width / 90.0;
        options.line_join = self.joints;
        options.end_cap = self.caps;
        options.start_cap = self.caps;
        let mut buffer: VertexBuffers<[f32; 3], u16> = VertexBuffers::new();
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
        Tessellation
        {
            colors: vec![self.color; buffer.vertices.len()],
            vertices: buffer.vertices,
            indices: Indices::U16(buffer.indices),
        }
    }
}
