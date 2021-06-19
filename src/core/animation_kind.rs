use crate::core::PackedAnimation;
use crate::draw::Tessellation;

pub trait AnimationKind
{
    fn generate
    (
        self,
        tesselation: Tessellation,
    ) -> PackedAnimation;
}