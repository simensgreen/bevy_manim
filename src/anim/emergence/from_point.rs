use crate::define_animation;
use crate::core::Animation;

define_animation!(FromPoint, FromPointAnimation, point: Vertex = [0.0, 0.0, 0.0]);

impl Animation for FromPointAnimation
{
    fn set_progress(&mut self, progress: f32)
    {
        self.data.progress = progress
    }

    fn get_progress(&self) -> f32
    {
        self.data.progress
    }

    fn get_vertices(&self) -> Vec<Vertex>
    {
        let mut vertices = Vec::with_capacity(self.tessellation.vertices.len());
        let point = self.data.point;
        let progress = (self.data.func)(self.data.progress);
        for vertex in self.tessellation.vertices.iter()
        {
            let dx = vertex[0] - point[0];
            let dy = vertex[1] - point[1];
            let dz = vertex[2] - point[2];
            vertices.push([dx * progress, dy * progress, dz * progress])
        }
        vertices
    }

    fn get_indices(&self) -> Indices
    {
        self.tessellation.indices.clone()
    }

    fn get_colors(&self) -> Vec<Color>
    {
        self.tessellation.colors.clone()
    }
}