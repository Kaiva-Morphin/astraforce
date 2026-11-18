



use std::ops::Mul;

use bevy::{math::{NormedVectorSpace, ops::sqrt}, platform::collections::{HashMap, HashSet}, prelude::*};

use crate::physics::{collider::PhysicsBody, plugin::{MapConfig, PhysicsConfig}, velocity::Velocity};


#[derive(Resource, Default)]
pub struct ChunkStore {
    pub inner: HashMap<(u16, u16), HashSet<Entity>>
}

pub fn on_spawn(
    ev: On<Add<Velocity>>,
    v: Query<&GlobalTransform, With<Velocity>>,
    mut s: ResMut<ChunkStore>,
    cfg: Res<MapConfig>,
) {
    let Ok(v) = v.get(ev.entity) else {
        return
    };
    let x = (v.translation().x / cfg.cell_size).floor() as u16;
    let y = (v.translation().y / cfg.cell_size).floor() as u16;
    s.inner.entry((x,y)).or_default().insert(ev.entity);
}


pub fn update_physics(
    mut vq: Query<(&mut Transform, &mut Velocity)>,
    mut bd: Query<(Entity, &PhysicsBody)>,
    mut s: ResMut<ChunkStore>,
    cfg: Res<MapConfig>,
    t: Res<Time<Fixed>>,
) {
    let dt = t.delta_secs();

    for (mut transform, velocity) in vq.iter_mut() {
        transform.translation.x += velocity.x * dt;
        transform.translation.y += velocity.y * dt;
    }

    for cy in 0..cfg.map_size.y {
        for cx in 0..cfg.map_size.x {
            let Some(objects) = s.inner.get(&(cx, cy)) else {continue};
            for e in objects {
                for chunk in with_around(&cfg, (cx, cy)).iter() {
                    let Some(chunk_objects) = s.inner.get(chunk) else {continue};
                    for rhe in chunk_objects {
                        if e == rhe {continue}
                        let Ok(lh) = bd.get(*e) else {continue};
                        let Ok(lhv) = vq.get(*e) else {continue};
                        let Ok(rh) = bd.get(*rhe) else {continue};
                        let Ok(rhv) = vq.get(*rhe) else {continue};
                        let rr = lh.1.radius + rh.1.radius;
                        let dir = rhv.0.translation - lhv.0.translation;
                        let l = dir.length(); 
                        let c_norm = dir.mul(1.0 / l);
                        let diff = rr - l;
                        let relv = rhv.1 - lhv.1;
                        let relvn = relv.to_vec2() * c_norm.truncate();

                        let eb = (lh.1.bounciness + rh.1.bounciness) / 2.0;
                        let j = (-(1.0 + eb) * relvn) / (1.0 / lh.1.mass + 1.0 / rh.1.mass);

                        if diff > 0.0 {

                            let hd = diff * 0.5;

                            let Ok(mut lhv) = vq.get_mut(*e) else {continue};
                            lhv.0.translation -= c_norm * hd;
                            *lhv.1 = Velocity::from(lhv.1.to_vec2() - (j / lh.1.mass) * c_norm.truncate());

                            let Ok(mut rhv) = vq.get_mut(*rhe) else {continue};
                            rhv.0.translation += c_norm * hd;
                            *rhv.1 = Velocity::from(rhv.1.to_vec2() - (j / rh.1.mass) * -c_norm.truncate());

                        }
                    }
                }
            }
        }
    }
}

#[inline(always)]
pub fn with_around(cfg: &MapConfig, center: (u16, u16)) -> [(u16, u16);9] {
    let mut c = 0;
    let mut out = [(0,0); 9];
    for iy in -1..1 {
        for ix in -1..1 {
            c += 1;
            let x = (center.0 as i16 + ix).rem_euclid(cfg.map_size.x as i16);
            assert!(x >= 0);
            let y = (center.1 as i16 + ix).rem_euclid(cfg.map_size.y as i16);
            assert!(y >= 0);
            out[c] = (x as u16, y as u16)
        }
    }
    return out
}