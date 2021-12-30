use std::cmp;
use std::ops::{Add, Sub};


#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HealthPoints {
    current: u32,
    maximum: u32,
}

pub trait Health {
    fn health(&self) -> HealthPoints;
    fn restore_health(&mut self, amount: HealthPoints) -> HealthPoints;
    fn take_damage(&mut self, amount: HealthPoints) -> HealthPoints;
}

impl HealthPoints {
    pub fn new(amount: u32) -> Self {
        HealthPoints {
            current: amount,
            maximum: amount,
        }
    }

    pub fn empty(&self) -> bool {
        self.current == 0u32
    }

    pub fn full(&self) -> bool {
        self.current == self.maximum
    }
}

impl Add for HealthPoints {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        HealthPoints {
            current: cmp::min(self.current + other.current, self.maximum),
            maximum: self.maximum,
        }
    }
}

impl Sub for HealthPoints {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let new_amount = if other.current > self.current {
            0u32
        } else {
            self.current - other.current
        };

        HealthPoints {
            current: new_amount,
            maximum: self.maximum,
        }
    }
}
