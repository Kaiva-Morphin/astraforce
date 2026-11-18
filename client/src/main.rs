use std::{f32::consts::PI, thread::current};

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use client_utils::prelude::*;
use rand::Rng;

use crate::physics::prelude::*;

pub mod physics;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "AstraForce".into(),
                ..default()
            }),
            ..default()
        })
        .set(ImagePlugin::default_nearest())
    )
    .add_plugins(EguiPlugin::default())
    .add_plugins(DebugOverlayPlugin::default())
    .add_plugins(PhysicsPlugin::default())
    .add_systems(Startup, setup)
    .add_systems(Update, update)
    .run();
}

const ZOOM : f32 = 3.0;
const MAP_SIZE : UVec2 = uvec2(10, 10);
const TILE_SIZE : f32 = 64.0;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Msaa::Off,
        Projection::Orthographic(
            OrthographicProjection {
                scaling_mode: bevy::camera::ScalingMode::AutoMax {
                    max_width: 1920f32 * 1.0 / ZOOM,
                    max_height: 1080f32 * 1.0 / ZOOM
                }, 
                ..OrthographicProjection::default_2d()
            }
        )
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("dbg_screen.png")),
        Transform::from_xyz(0., 0., 0.),
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("ships/alpha.png")),
        Transform::from_xyz(0., 0., 0.),
        Player,
        PhysicsBody::radius(12.0),
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("ships/alpha.png")),
        Transform::from_xyz(40., 0., 0.),
        Velocity::from(vec2(-5.0, 0.0)),
        PhysicsBody::radius(12.0),
    ));
}

#[derive(Component)]
pub struct Player;



fn update(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut player: Single<(&mut Transform, &mut Velocity), (With<Player>, Without<Camera>)>,
    mut camera_query: Single<(&Camera, &GlobalTransform, &mut Transform), Without<Player>>,
    t: Res<Time>,
    window: Single<&Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut gizmos: Gizmos,
){
    let (camera, camera_gt, camera_t) = &mut *camera_query;
    let dt = t.delta_secs();
    
    if let Some(cursor_position) = window.cursor_position()
        && let Ok(world_pos) = camera.viewport_to_world_2d(camera_gt, cursor_position)
    {
        gizmos.circle_2d(world_pos, 10., bevy::color::palettes::css::WHITE);

        let (pt, vel) = &mut *player;
            
        let player_pos_2d = pt.translation.truncate();
        
        let dir = world_pos - player_pos_2d;
        let target_angle = -dir.angle_to(Vec2::Y);

        let current_angle = pt.rotation.to_euler(EulerRot::ZXY).0;

        let mut angle_diff = target_angle - current_angle;
        while angle_diff >  std::f32::consts::PI { angle_diff -= std::f32::consts::TAU; }
        while angle_diff < -std::f32::consts::PI { angle_diff += std::f32::consts::TAU; }

        let smoothness = 5.0;
        let max_step = 0.1;
        
        let eased_step = angle_diff * (1.0 - (-smoothness * dt).exp());
        
        let new_angle = current_angle + eased_step.clamp(-max_step, max_step);

        pt.rotation = Quat::from_rotation_z(new_angle);

        gizmos.line_2d(player_pos_2d, player_pos_2d + dir.normalize() * 25.0, bevy::color::palettes::css::RED);

        let forward = Vec2::Y.rotate_angle(current_angle);

        gizmos.line_2d(player_pos_2d, player_pos_2d + forward * 25.0, bevy::color::palettes::css::BLUE);

        if mouse.pressed(MouseButton::Right) 
        {
            **vel += Vec2::Y.rotate_angle(current_angle) * 50.0 * dt;
            cmd.spawn((
                Transform::from_translation(-forward.extend(0.0) * 5.0 + player_pos_2d.extend(0.0)),
                Velocity::from(-forward * 200.0),
                Sprite::from_image(asset_server.load("particles/small.png")),
            ));
            
        }

        camera_t.translation = pt.translation;
    }

}