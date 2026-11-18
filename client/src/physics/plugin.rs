use bevy::{math::{U16Vec2, u16vec2}, prelude::*};

use crate::physics::{chunks, prelude::*};
use bevy::color::palettes::css::*;

pub struct PhysicsPlugin {
    pub tps: f32,
    pub cell_size: f32,
}

#[derive(Resource)]
pub struct MapConfig {
    pub cell_size: f32,
    pub map_size: U16Vec2,
}

impl Default for MapConfig {
    fn default() -> Self {
        Self {
            cell_size: 50.0,
            map_size: u16vec2(4, 3)
        }
    }
}

#[derive(Resource, Default)]
pub struct PhysicsConfig {
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
            .add_systems(FixedUpdate, chunks::update_physics)
            .add_systems(Update, update)
            .add_systems(Update, draw_debug.run_if(|r: Res<PhysicsConfig>| r.debug_enabled))
            .init_resource::<MapConfig>()
            .init_resource::<PhysicsConfig>()
            .init_resource::<ChunkStore>()
            .add_observer(chunks::on_spawn)
            ;
    }
}






fn draw_debug(
    mut gizmos: Gizmos,
    mut bodies: Query<(&GlobalTransform, Entity, &PhysicsBody)>,
    mut velocities: Query<(&GlobalTransform, &Velocity)>,
){
    for (gt, vel) in velocities {
        let s = gt.translation().truncate();
        let v : Vec2 = vel.into();
        gizmos.arrow_2d(s, s + v, GREEN);
    }
    for (gt, e, b) in bodies {
        gizmos.circle_2d(Isometry2d::from_translation(gt.translation().truncate()), b.radius, ORANGE);
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

