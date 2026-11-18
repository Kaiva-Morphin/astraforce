use bevy::prelude::*;

use crate::physics::velocity::Velocity;



#[derive(Component)]
#[require(
    Velocity,
    Transform,
)]
pub struct PhysicsBody {
    pub radius: f32,
    pub mass: f32,
    pub bounciness: f32,
}

impl PhysicsBody {
    pub fn radius(r: f32) -> Self {
        Self {
            radius: r, 
            ..default()
        }
    }
}

impl Default for PhysicsBody {
    fn default() -> Self {
        Self {
            radius: 1.0,
            mass: 1.0,
            bounciness: 1.0,
        }
    }
}



pub struct CollisionLayers { // todo!
    mask: u8

}

