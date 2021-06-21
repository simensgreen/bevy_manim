use crate::core::PackedAnimation;
use crate::draw::Tessellation;

pub trait AnimationKind: Send + Sync + 'static + Clone
{
    fn generate
    (
        self,
        tesselation: Tessellation,
    ) -> PackedAnimation;
}