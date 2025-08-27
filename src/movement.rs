use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use std::f32::consts::FRAC_PI_2;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub sensitivity: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 5.0,
            // Reasonable sensitivity for smooth mouse look
            sensitivity: 0.5,
        }
    }
}

#[derive(Component)]
pub struct CameraController {
    pub yaw: f32,
    pub pitch: f32,
}

pub fn setup_camera_controller(
    mut commands: Commands,
    camera_query: Query<Entity, (With<Camera3d>, Without<CameraController>)>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    for entity in camera_query.iter() {
        commands.entity(entity).insert((
            Player::default(),
            CameraController {
                yaw: 0.0,
                pitch: 0.0,
            },
        ));
    }

    // Auto-grab cursor
    if let Ok(mut window) = window_query.get_single_mut() {
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
    }
}

// Movement system
pub fn move_camera(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Player, &CameraController)>,
) {
    for (mut transform, player, controller) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        // Calculate forward/right vectors based on yaw, ignoring pitch
        let (yaw_sin, yaw_cos) = controller.yaw.sin_cos();
        let forward = Vec3::new(yaw_sin, 0.0, yaw_cos);
        let right = Vec3::new(yaw_cos, 0.0, -yaw_sin);

        // AZERTY + QWERTY support
        if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::KeyZ) {
            direction += forward;
        }
        if input.pressed(KeyCode::KeyS) {
            direction -= forward;
        }
        if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::KeyQ) {
            direction -= right;
        }
        if input.pressed(KeyCode::KeyD) {
            direction += right;
        }
        if input.pressed(KeyCode::Space) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::ShiftLeft) {
            direction.y -= 1.0;
        }

        if direction.length_squared() > 0.0 {
            transform.translation += direction.normalize() * player.speed * time.delta_secs();
        }
    }
}

// Cursor position-based mouse look system
pub fn look_around(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&mut Transform, &Player, &mut CameraController)>,
) {
    if let Ok(window) = window_query.get_single() {
        if let Some(cursor_pos) = window.cursor_position() {
            let window_size = Vec2::new(window.width(), window.height());
            let center = window_size / 2.0;
            
            // Calculate offset from center
            let offset = cursor_pos - center;
            
            for (mut transform, player, mut controller) in query.iter_mut() {
                // Convert cursor position to rotation angles
                controller.yaw = -offset.x * player.sensitivity * 0.01;
                controller.pitch = -offset.y * player.sensitivity * 0.01;
                
                // Clamp pitch to prevent looking straight up or down
                controller.pitch = controller.pitch.clamp(-FRAC_PI_2 + 0.1, FRAC_PI_2 - 0.1);
                
                // Apply rotation
                transform.rotation = Quat::from_axis_angle(Vec3::Y, controller.yaw)
                    * Quat::from_axis_angle(Vec3::X, controller.pitch);
            }
        }
    }
}

pub fn cursor_grab(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = window_query.get_single_mut() {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            window.cursor_options.grab_mode = CursorGrabMode::Locked;
            window.cursor_options.visible = false;
        }
    }
}

pub fn debug_camera_info(
    input: Res<ButtonInput<KeyCode>>,
    query: Query<(&Transform, &CameraController)>,
) {
    if input.just_pressed(KeyCode::F1) {
        for (transform, controller) in query.iter() {
            println!(
                "Camera Position: {:?}
Yaw: {:.2}° Pitch: {:.2}°",
                transform.translation,
                controller.yaw.to_degrees(),
                controller.pitch.to_degrees()
            );
        }
    }
}
