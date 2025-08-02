use bevy::prelude::*;
use echo::movement::{move_camera, Player};

#[test]
fn test_move_camera() {
    // Setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_systems(Update, move_camera);

    let camera_transform = Transform::from_xyz(0.0, 0.0, 0.0);
    let camera_entity = app.world.spawn((
        Camera3d::default(), 
        camera_transform.clone(), 
        Player::default(),
    )).id();

    let mut input = Input::<KeyCode>::default();
    input.press(KeyCode::W); // Use W key which is handled by our movement system
    app.insert_resource(input);

    // Run update once to initialize everything, including Time
    app.update();

    // Manually advance time
    let mut time = app.world.resource_mut::<Time>();
    time.advance_by(std::time::Duration::from_secs_f32(0.1));

    // Run update again to see the effect of the time change
    app.update();

    // Check
    let new_transform = app.world.get::<Transform>(camera_entity).unwrap();
    assert_ne!(new_transform.translation, camera_transform.translation, "Camera should have moved");
}

