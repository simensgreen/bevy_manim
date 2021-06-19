use crate::core::{Vertex, Color, Indices};
use std::f32::consts::PI;
use crate::draw::{Drawing, Tessellation};

pub struct Circle
{
    pub center: Vertex,
    pub radius: f32,
    pub width: f32,
    pub color: Color,
}

impl Default for Circle
{
    fn default() -> Self
    {
        Self
        {
            center: [0.0, 0.0, 0.0],
            radius: 1.0,
            width: 0.1,
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}

impl Drawing for Circle
{
    fn get_tessellation(&self) -> Tessellation
    {
        let circle_len = PI * 2.0 * self.radius;
        let point_count = (circle_len / self.width / 90.0).ceil() as u16 * 2;
        let mut vertices = Vec::with_capacity(point_count as usize);
        let mut indices = Vec::with_capacity(point_count as usize);
        let mut colors = Vec::with_capacity(point_count as usize);
        let angle_step = PI * 2.0 / point_count as f32;
        for i in 0..point_count
        {
            let angle = i as f32 * angle_step;
            let radius = if i % 2 == 0 { self.radius + self.width / 2.0 } else { self.radius - self.width / 2.0};
            let x = radius * angle.sin();
            let y = radius * angle.cos();
            vertices.push([x, y, 0.0]);
        }
        for i in 0..point_count
        {
            indices.push(i);
            if i % 2 == 0
            {
                indices.push((i + 1) % point_count);
                indices.push((i + 2) % point_count);
            }
            else
            {
                indices.push((i + 2) % point_count);
                indices.push((i + 1) % point_count);
            }
            colors.push(self.color.clone());
        }
        Tessellation
        {
            vertices, indices: Indices::U16(indices), colors
        }
    }
}