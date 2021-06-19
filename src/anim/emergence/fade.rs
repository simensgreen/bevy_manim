use crate::define_animation;
use crate::core::Animation;


define_animation!(Fade, FadeAnimation);


impl Animation for FadeAnimation
{
    #[inline]
    fn set_progress(&mut self, progress: f32) { self.data.progress = progress }

    #[inline]
    fn get_progress(&self) -> f32 { self.data.progress }

    fn get_vertices(&self) -> Vec<Vertex>
    {
        self.tessellation.vertices.clone()
    }

    fn get_indices(&self) -> Indices
    {
        self.tessellation.indices.clone()
    }

    fn get_colors(&self) -> Vec<Color>
    {
        let progress = (self.data.func)(self.data.progress);
        let mut colors = Vec::with_capacity(self.tessellation.colors.len());
        for color in self.tessellation.colors.iter()
        {
            let new_color = [color[0], color[1], color[2], color[3] * progress];
            colors.push(new_color);
        }
        colors
    }
}