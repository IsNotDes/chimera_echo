use bevy::prelude::*;
use crate::movement::{Player, CameraController};
use bevy::window::{CursorGrabMode, PrimaryWindow};

pub fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // Plane - using new Mesh3d and MeshMaterial3d components
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::default(),
    ));

    // Light - using new PointLight component
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Camera with Player and CameraController components
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.0, 0.0).looking_at(Vec3::new(1.0, 1.0, 1.0), Vec3::Y),
        Player::default(),
        CameraController {
            yaw: 0.0,
            pitch: 0.0,
        },
    ));
}

// Setup cursor grab in a separate system that runs after the window is ready
pub fn setup_cursor_grab(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = window_query.get_single_mut() {
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
    }
}