use std::marker::PhantomData;

use super::units;

use chrono::Duration;
use ggez::event::KeyCode;


#[derive(Copy, Clone)]
pub struct Position<U: Copy>{
    pub x: U,
    pub y: U,
}

#[derive(Copy, Clone)]
pub struct Dimensions<U: Copy> {
    pub width: U,
    pub height: U,
}

/// An option-like representation of the directions of arrow keys.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Stationary,
}

/// Represents directional motion in some specified units.
pub struct Velocity<U>(f32, PhantomData<U>);

pub trait Object<U: Copy> {
    fn position(&self) -> Position<U>;
    fn dimensions(&self) -> Dimensions<U>;
}

pub trait Acceleration<U> {
    fn horizontal_velocity(&self, time: Duration) -> Velocity<U>;
    fn vertical_velocity(&self, time: Duration) -> Velocity<U>;
}

impl Direction {
    pub fn is_horizontal(&self) -> bool {
        *self == Direction::Left || *self == Direction::Right
    }

    pub fn is_vertical(&self) -> bool {
        *self == Direction::Down || *self == Direction::Up
    }
}

impl<U: Copy> Velocity<U> {
    pub fn new(value: f32) -> Self {
        Velocity(value, PhantomData)
    }
}

impl Velocity<units::PixelsPerMs> {
    pub fn distance(&self, time: Duration) -> units::Pixels {
        units::Pixels(self.0 * time.num_milliseconds() as f32)
    }
}

impl<U: Copy> Position<U> {
    pub fn new(x: U, y: U) -> Self {
        Position {
            x: x,
            y: y,
        }
    }
}

impl<U: Copy> Dimensions<U> {
    pub fn new(width: U, height: U) -> Self {
        Dimensions {
            width: width,
            height: height,
        }
    }
}
