use std::marker::PhantomData;

use crate::{
    game::bullets::{
        self,
        PlayerBullet,
    },
    game::player::Player,
    physics::units,
};

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
    fn horizontal_velocity(dir: Direction, time: Duration) -> Velocity<U>;
    fn vertical_velocity(dir: Direction, time: Duration) -> Velocity<U>;
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

impl Acceleration<units::PixelsPerMs> for Player {
    fn horizontal_velocity(direction: Direction, _time: Duration) -> Velocity<units::PixelsPerMs> {
        match direction {
            Direction::Left  => Velocity(-0.5, PhantomData),
            Direction::Right => Velocity(0.5, PhantomData),
            _                => Velocity(0.0, PhantomData),
        }
    }

    fn vertical_velocity(direction: Direction, _time: Duration) -> Velocity<units::PixelsPerMs> {
        match direction {
            Direction::Up   => Velocity(-0.5, PhantomData),
            Direction::Down => Velocity(0.5, PhantomData),
            _               => Velocity(0.0, PhantomData),
        }
    }
}

impl Acceleration<units::PixelsPerMs> for PlayerBullet {
    fn horizontal_velocity(direction: Direction, _time: Duration) -> Velocity<units::PixelsPerMs> {
        Velocity(0.0, PhantomData)
    }

    fn vertical_velocity(direction: Direction, _time: Duration) -> Velocity<units::PixelsPerMs> {
        Velocity(-2.0, PhantomData)
    }
}

impl Acceleration<units::PixelsPerMs> for bullets::Basic {
    fn horizontal_velocity(direction: Direction, _time: Duration) -> Velocity<units::PixelsPerMs> {
        Velocity(0.0, PhantomData)
    }

    fn vertical_velocity(direction: Direction, _time: Duration) -> Velocity<units::PixelsPerMs> {
        Velocity(0.5, PhantomData)
    }
}

impl Velocity<units::PixelsPerMs> {
    pub fn distance(&self, time_passed: Duration) -> units::Pixels {
        units::Pixels(self.0 * time_passed.num_milliseconds() as f32)
    }
}
