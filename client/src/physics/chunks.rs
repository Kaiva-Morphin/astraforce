



use bevy::{platform::collections::HashMap, prelude::*};

use crate::physics::{plugin::PhysicsConfig, velocity::Velocity};


#[derive(Resource)]
pub struct ChunkStore {
    pub inner: Vec<Vec<Entity>>
}

pub fn on_spawn(
    ev: On<Add<Velocity>>,
    v: Query<&GlobalTransform, With<Velocity>>,
    mut s: ResMut<ChunkStore>,
    cfg: Res<PhysicsConfig>,
) {
    let Ok(v) = v.get(ev.entity) else {
        return
    };
    let x = (v.translation().x / cfg.cell_size).floor() as u16;
    let y = (v.translation().y / cfg.cell_size).floor() as u16;
    e.push(ev.entity);
}

fn update(

) {

}
