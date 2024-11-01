use bevy::{
    input::{
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    },
    prelude::*,
};
use bevy_rapier3d::prelude::*;

use crate::{common::commands::{action_from_input, Action}, game::{player::{components::{Player, PlayerFirstPersonCamera}, constants::{PLAYER_JUMP, PLAYER_SPEED}}, weapons::ak74::components::AK74Component}};

pub fn player_look_system(
    windows: Query<&Window>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<PlayerFirstPersonCamera>, Without<Player>)>,

    mut mouse_motion: EventReader<MouseMotion>,
) {
    let window = windows.single();
    let mut player_transform = player_query.single_mut();
    let mut camera_transform = camera_query.single_mut();

    let mut total_yaw = 0.0f32;
    let mut total_pitch = 0.0f32;

    for ev in mouse_motion.read() {
        total_yaw -= ev.delta.x * 0.005;
        total_pitch -= ev.delta.y * 0.005;
    }

    // Rotate the player around the Y axis
    player_transform.rotate_y(total_yaw);

    // Update the camera pitch
    let (yaw, mut pitch, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);
    pitch += total_pitch;
    pitch = pitch.clamp(-1.54, 1.54); // Approximately -88 to +88 degrees
    camera_transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
}

pub fn player_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut velocity, mut transform)) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        let actions = action_from_input(&keyboard_input);
        
        let forward = transform.forward();
        let right = transform.right();
        
        let mut speed = PLAYER_SPEED;

        
        for action in actions {
            match action {
                Action::WalkForward => direction += *forward,
                Action::WalkBackward => direction -= *forward,
                Action::WalkLeftward => direction -= *right,
                Action::WalkRightward => direction += *right,
                Action::Run => speed *= 2.0,
                Action::Jump => {
                    // Handle jump
                    if velocity.linvel.y.abs() < 0.1 {
                        velocity.linvel.y = PLAYER_JUMP;
                    }
                }
     
                _ => ()
            }
        }
        
        if direction != Vec3::ZERO {
            direction = direction.normalize();
            let movement = direction * speed * time.delta_seconds();
            
            transform.translation += direction * speed * time.delta_seconds();
            // velocity.linvel += Vec3::new(movement.x, 0.0, movement.z);
        }
        
        // Apply drag to slow down the player when no input is given
        // velocity.linvel *= 0.9;
        
       
    }
}

pub fn player_movement_editor_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform), With<Player>>, // Removed Velocity
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        // Get movement actions
        let actions = action_from_input(&keyboard_input);
        
        let forward = transform.forward();
        let right = transform.right();
        let mut speed = PLAYER_SPEED;

        for action in actions {
            match action {
                Action::WalkForward => direction += *forward,
                Action::WalkBackward => direction -= *forward,
                Action::WalkLeftward => direction -= *right,
                Action::WalkRightward => direction += *right,
                Action::Run => speed *= 2.0,
                Action::Crouch => transform.translation.y -= 500.0 * time.delta_seconds(),
                Action::Jump => {
                    transform.translation.y += 500.0 * time.delta_seconds();
                },
                _ => (),
            }
        }

        // Apply movement based on direction
        if direction != Vec3::ZERO {
            direction = direction.normalize();
            transform.translation += direction * speed * time.delta_seconds();
        }
    }
}
