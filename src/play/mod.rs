use std::time::Duration;
use crate::core::PackedAnimation;

#[derive(Clone)]
pub struct ScheduledAnimation
{
    pub is_loop: bool,
    pub start: f32,
    pub duration: Duration,
    pub animation: PackedAnimation
}
