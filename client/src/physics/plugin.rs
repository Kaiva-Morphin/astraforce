use bevy::{math::U16Vec2, prelude::*};

use crate::physics::{chunks, prelude::*};
use bevy::color::palettes::css::*;

pub struct PhysicsPlugin {
    pub tps: f32,
    pub cell_size: f32,
}

#[derive(Resource, Default)]
pub struct PhysicsConfig {
    pub map_size: U16Vec2,
    pub cell_size: u16,
    pub debug_enabled: bool,
}

impl Default for PhysicsPlugin {
    fn default() -> Self {
        Self {
            tps: 30.0, // todo! set fixed steps
            cell_size: 10.0,
        }
    }
}


impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(FixedUpdate, update_physics)
            .add_systems(Update, update)
            .add_systems(Update, draw_debug.run_if(|r: Res<PhysicsConfig>| r.debug_enabled))
            .init_resource::<PhysicsConfig>()
            .add_observer(chunks::on_spawn)
            ;
    }
}






fn draw_debug(
    mut gizmos: Gizmos,
    mut colliders: Query<(&GlobalTransform, Entity, &Collider)>,
    mut velocities: Query<(&GlobalTransform, &Velocity)>,
    mut bodies: Query<&PhysicsBody>
){
    for (gt, vel) in velocities {
        let s = gt.translation().truncate();
        let v : Vec2 = vel.into();
        gizmos.arrow_2d(s, s + v, GREEN);
    }
    for (gt, e, c) in colliders {
        let b = bodies.get(e).ok();
        
        let color = if let Some(v) = b {
            match v {
                PhysicsBody::Fixed => RED,
                PhysicsBody::Dynamic => ORANGE
            }
        } else {
            GRAY
        };

        let p = gt.translation().truncate();
        match c {
            Collider::CircleCollider { radius } => {
                gizmos.circle_2d(Isometry2d::from_translation(p), *radius, color);
            },
            Collider::LineCollider { start: a, end: b } => {
                gizmos.line_2d(p + a, p + b, color);
            }
        }
    }
}

fn update(
    k: Res<ButtonInput<KeyCode>>,
    mut cfg: ResMut<PhysicsConfig>,
) {
    if k.just_pressed(KeyCode::F2) {
        cfg.debug_enabled = !cfg.debug_enabled;
    }
}

fn update_physics(
    mut vq: Query<(&mut Transform, &mut Velocity)>,
    mut bd: Query<(Entity, &PhysicsBody)>,
    t: Res<Time<Fixed>>,
) {
    let dt = t.delta_secs();

    for (mut transform, velocity) in vq.iter_mut() {
        transform.translation.x += velocity.x * dt;
        transform.translation.y += velocity.y * dt;
    }
}
