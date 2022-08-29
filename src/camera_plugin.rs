use crate::common::MAP_SIZE;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(camera_move)
            .add_system(camera_reset);
    }
}

const CAMERA_SPEED: f32 = 800.;
const CAMERA_ZOOM_SPEED: f32 = 5.;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn camera_move(
    keyboard_res: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut q: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    let (mut camera_transform, mut camera_orth) = q.single_mut();

    let mut v = Vec2::default();
    if keyboard_res.pressed(KeyCode::A) {
        v.x -= CAMERA_SPEED * time.delta_seconds();
    }
    if keyboard_res.pressed(KeyCode::S) {
        v.y -= CAMERA_SPEED * time.delta_seconds();
    }
    if keyboard_res.pressed(KeyCode::D) {
        v.x += CAMERA_SPEED * time.delta_seconds();
    }
    if keyboard_res.pressed(KeyCode::W) {
        v.y += CAMERA_SPEED * time.delta_seconds();
    }

    let (min_orth, max_orth) = (0.5, 2.5);
    if keyboard_res.pressed(KeyCode::Z) {
        camera_orth.scale += CAMERA_ZOOM_SPEED * time.delta_seconds();
        camera_orth.scale = camera_orth.scale.clamp(min_orth, max_orth);
    }
    if keyboard_res.pressed(KeyCode::X) {
        camera_orth.scale -= CAMERA_ZOOM_SPEED * time.delta_seconds();
        camera_orth.scale = camera_orth.scale.clamp(min_orth, max_orth);
    }

    camera_transform.translation += v.extend(0.);
}

fn camera_reset(keyboard_res: Res<Input<KeyCode>>, mut q: Query<&mut Transform, With<Camera>>) {
    if keyboard_res.pressed(KeyCode::C) {
        let mut camera_transform = q.single_mut();
        camera_transform.translation = Vec3::new(
            MAP_SIZE as f32 / 2.,
            MAP_SIZE as f32 / 2.,
            camera_transform.translation.z,
        );
    }
}
