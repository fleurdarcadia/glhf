/// A unit of distance.
#[derive(Copy, Clone)]
pub struct Pixels(pub f32);

/// A unit representing the number of pixels that an object moves across the screen
/// per millisecond between updates.
#[derive(Copy, Clone)]
pub struct PixelsPerMs;

impl Pixels {
    #[inline(always)]
    pub fn value(&self) -> f32 {
        self.0
    }
}
