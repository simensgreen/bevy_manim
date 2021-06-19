#[inline]
pub fn linear(t: f32) -> f32 { t }

#[inline]
pub fn smooth(t: f32) -> f32
{ let s = 1.0 - t; (t * t * t) * (10.0 * s * s + 5.0 * s * t + t * t) }

#[inline]
pub fn slow_into(t: f32) -> f32 { (1.0 - (1.0 - t) * (1.0 - t)).sqrt() }

#[inline]
pub fn rush_into(t: f32) -> f32 { 2.0 * smooth(0.5 * t) }

#[inline]
pub fn rush_from(t: f32) -> f32 { 2.0 * smooth(0.5 * (t + 1.0)) - 1.0 }

#[inline]
pub fn double_smooth(t: f32) -> f32
{
    if t < 0.5 { 0.5 * smooth(2.0 * t) } else { 0.5 * (1.0 + smooth(2.0 * t - 1.0)) }
}