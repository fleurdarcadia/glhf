use std::marker::PhantomData;

use super::units;

use chrono::Duration;
use ggez::event::KeyCode;


/// An option-like representation of the directions of arrow keys.
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Stationary,
}

/// Represents directional motion in some specified units.
pub struct Velocity<U>(f32, PhantomData<U>);

impl Velocity<units::PixelsPerMs> {
    pub fn horizontal(direction: &Direction) -> Self {
        match direction {
            Direction::Left  => Velocity(-0.5, PhantomData),
            Direction::Right => Velocity(0.5, PhantomData),
            _                => Velocity(0.0, PhantomData),
        }
    }

    pub fn vertical(direction: &Direction) -> Self {
        match direction {
            Direction::Up   => Velocity(-0.5, PhantomData),
            Direction::Down => Velocity(0.5, PhantomData),
            _               => Velocity(0.0, PhantomData),
        }
    }

    pub fn distance(&self, time_passed: Duration) -> units::Pixels {
        units::Pixels(self.0 * time_passed.num_milliseconds() as f32)
    }
}
