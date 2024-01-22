use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::render::camera::ScalingMode;
use crate::camera::components::CameraMaker;
use crate::MyRobot;
use crate::player::components::Player;
use bevy::core_pipeline::clear_color::ClearColorConfig;

pub const CAMERA_SPEED : f32 = 250.0;

pub fn zoom_scalingmode(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut OrthographicProjection, With<CameraMaker>>,
) { 

    if keyboard_input.pressed(KeyCode::P) {
        let mut projection = camera_query.single_mut();
        // Zoom in 
        projection.scale /= 1.03;
    }

    if keyboard_input.pressed(KeyCode::Minus) {
        let mut projection = camera_query.single_mut();
        // Zoom in 
        projection.scale *= 1.03;
    }

 }

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let mut camera = Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    };
    // Zooming 
    camera.projection.scale /= 5.0;
    camera.camera_2d.clear_color = ClearColorConfig::Custom(Color::rgba_u8(31, 19, 2, 1));
    commands.spawn(
        (
            camera,   
            CameraMaker{},
        )
    );
}

pub fn follow_robot(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_transform_query: Query<&mut Transform, With<CameraMaker>>,
    robot_transform_query: Query<&Transform, (With<Player>, Without<CameraMaker>)>,
) {
    if keyboard_input.pressed(KeyCode::F) {
        let mut camera_transform = camera_transform_query.get_single_mut().unwrap();
        let robot_transform = robot_transform_query.get_single().unwrap();

        camera_transform.translation.x = robot_transform.translation.x;
        camera_transform.translation.y = robot_transform.translation.y;
    }
}


pub fn camera_movement(
    keyboard_input: Res<Input<KeyCode>>,
    // A query matches only the entities that fit the specification
    // It returns an iterator
    mut camera_query: Query<&mut Transform, With<CameraMaker>>,
    time: Res<Time>,
)
{
    // The player might not exist
    if let Ok(mut transform) = camera_query.get_single_mut() {

        // We are using a direction vector so get the overall direction
        let mut direction = Vec3::ZERO;

        // This is how you register keyboard input
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        // If the direction vector is not null
        if direction.length() > 0.0 {
            // We normalize to make sure that the diagonals move as far
            // as horizontal or vertical axes movement
            direction = direction.normalize();
        }

        // Adding the direction
        transform.translation += direction * CAMERA_SPEED * time.delta_seconds();
    }
}

