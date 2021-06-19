pub mod emergence;
pub mod highlight;
pub mod delete;
pub mod transform;
pub mod funcs;

#[macro_export]
macro_rules! define_animation
{
    ($name:ident, $anim_name:ident $(,)?
    $($field:ident: $ftype:ty = $fdef:expr),* $(,)?) =>
    {
        use $crate::core::
        {
            ProgressFunc,
            Vertex,
            Indices,
            Color,
            AnimationKind,
            PackedAnimation,
            DEFAULT_PROGRESS_FUNC
        };
        use $crate::draw::Tessellation;
        use std::sync::{Arc, Mutex};

        #[derive(Clone, Debug)]
        pub struct $name
        {
            pub progress: f32,
            pub func: ProgressFunc,
            $(
                pub $field: $ftype,
            )*
        }

        impl Default for $name
        {
            fn default() -> Self
            {
                Self
                {
                    progress: 0.0,
                    func: DEFAULT_PROGRESS_FUNC,
                    $(
                        $field: $fdef,
                    )*
                }
            }
        }

        impl AnimationKind for $name
        {
            fn generate
                (
                    self,
                    tessellation: Tessellation
                ) -> PackedAnimation
                {
                    Arc::new(
                    Mutex::new(
                        $anim_name
                        {
                            tessellation, data: self
                        }
                    ))
                }
        }

        #[derive(Debug, Clone, Default)]
        pub(crate) struct $anim_name
        {
            tessellation: Tessellation,
            data: $name,
        }

    };
}

pub mod dummy
{
    use crate::core::Animation;

    define_animation!(Dummy, DummyAnimation);

    impl Animation for DummyAnimation
    {
        #[inline]
        fn set_progress(&mut self, progress: f32) { self.data.progress = progress }

        #[inline]
        fn get_progress(&self) -> f32 { self.data.progress }

        #[inline]
        fn get_vertices(&self) -> Vec<Vertex> { self.tessellation.vertices.clone() }

        #[inline]
        fn get_indices(&self) -> Indices { self.tessellation.indices.clone() }

        #[inline]
        fn get_colors(&self) -> Vec<Color> { self.tessellation.colors.clone() }
    }
}