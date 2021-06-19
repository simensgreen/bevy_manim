use crate::core::{Vertex, Indices, Color};

pub trait Animation: Send + Sync
{
    fn set_progress(&mut self, progress: f32);
    fn get_progress(&self) -> f32;
    fn get_vertices(&self) -> Vec<Vertex>;
    fn get_indices(&self) -> Indices;
    fn get_colors(&self) -> Vec<Color>;
}
