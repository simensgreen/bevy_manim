use std::sync::{Arc, Mutex};


pub(crate) type ProgressFunc = fn(f32) -> f32;
pub(crate) type Color = [f32; 4];
pub(crate) type Indices = bevy::render::mesh::Indices;
pub(crate) type Vertex = [f32; 3];
pub type PackedAnimation = Arc<Mutex<dyn Animation>>;

use crate::anim::funcs::{smooth};
pub(crate) const DEFAULT_PROGRESS_FUNC: ProgressFunc = smooth;

mod animation_kind;
mod animation;

pub use animation_kind::AnimationKind;
pub use animation::Animation;
