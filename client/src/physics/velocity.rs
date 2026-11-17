use std::ops::{Add, AddAssign};
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

fn vel2(x: f32, y: f32) -> Velocity {
    Velocity { x, y }
}

impl From<Vec2> for Velocity {
    fn from(value: Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl Add for Velocity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Vec2> for Velocity {
    type Output = Velocity;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Vec2> for Velocity {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// impl Into<Vec2> for Velocity {
//     fn into(self) -> Vec2 {
//         Vec2 {
//             x: self.x,
//             y: self.y
//         }
//     }
// }

impl From<Velocity> for Vec2 {
    fn from(value: Velocity) -> Self {
        Vec2 {
            x: value.x,
            y: value.y
        }
    }
}

impl From<&Velocity> for Vec2 {
    fn from(value: &Velocity) -> Self {
        Vec2 {
            x: value.x,
            y: value.y
        }
    }
}