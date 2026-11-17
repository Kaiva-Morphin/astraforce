use bevy::prelude::*;



#[derive(Component, Default)]
pub enum PhysicsBody {
    Fixed,
    #[default]
    Dynamic
}

#[derive(Component)]
pub enum Collider {
    LineCollider {
        start: Vec2,
        end: Vec2,
    },
    CircleCollider {
        radius: f32,
    }
}


impl Collider {
    pub fn circle(r: f32) -> Self {
        Self::CircleCollider {radius: r}
    }
}



pub struct CollisionLayers { // todo!
    mask: u8

}

