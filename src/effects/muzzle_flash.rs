use bevy::prelude::*;

use crate::{common::states::CurrentWeapon, game::{player::components::{Player, PlayerFirstPersonCamera}, weapons::{ak74::components::AK74Component, common::can_shoot_and_decrease_ammo, glock::components::GlockComponent}}};

use super::components::{HasMuzzleFlash, MuzzleFlash};

pub fn setup_muzzle_flash(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let muzzle_flash_handle = asset_server.load("models/muzzle-flash.glb#Scene0");

    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).with_children(|parent| {
            parent.spawn((
                SceneBundle {
                    scene: muzzle_flash_handle.clone(),
                    transform: Transform {
                        scale: Vec3::splat(0.2),
                        translation: Vec3::new(90.2, 8.85, -200.0),
                        ..default()
                    },
                      visibility: Visibility::Hidden, 
                    ..default()
                },
                MuzzleFlash {
                    timer: Timer::from_seconds(0.02, TimerMode::Once),
                    is_active: false,
                    frames_visible: 0
                },
            ));
        });
    }
}


pub fn update_muzzle_flash(
    mut commands: Commands,
    time: Res<Time>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut muzzle_flash_query: Query<(Entity, &mut MuzzleFlash, &mut Visibility, &mut Transform)>,
    camera_query: Query<&Transform, (With<PlayerFirstPersonCamera>, Without<MuzzleFlash>)>,
    weapon_state: Res<State<CurrentWeapon>>,
    mut glock_query: Query<&mut GlockComponent>,
    mut ak74_query: Query<&mut AK74Component>,
) {
    // Get the camera transform
    let camera_transform = if let Ok(transform) = camera_query.get_single() {
        transform
    } else {
        return;
    };

    
    let forward_vector = camera_transform.forward();
    let camera_pitch = forward_vector.y;

    let cam = camera_transform.rotation;


    // Check if the left mouse button is pressed
    if mouse_input.just_pressed(MouseButton::Left) {
        let can_shoot = match weapon_state.get() {
            CurrentWeapon::Glock => {
                if let Ok(mut glock) = glock_query.get_single_mut() {
                    if glock.current_ammo > 0 {
                        true // Can shoot
                    } else {
                        false // Out of ammo
                    }
                } else {
                    false // No weapon found
                }
            }
            CurrentWeapon::AK74 => {
                if let Ok(mut ak74) = ak74_query.get_single_mut() {
                    if ak74.current_ammo > 0 {
                        true // Can shoot
                    } else {
                        false // Out of ammo
                    }
                } else {
                    false // No weapon found
                }
            }
            CurrentWeapon::None => false,
        };

       
       if can_shoot {
        for (_, mut muzzle_flash, mut visibility, mut transform) in muzzle_flash_query.iter_mut() {
            // Reset the muzzle flash timer
            muzzle_flash.timer.reset();
            muzzle_flash.is_active = true;
            muzzle_flash.frames_visible = 0;
            *visibility = Visibility::Visible;
            println!("Updated muzzle flash");
            let mut new_y_pos = 86.0;
            let mut new_z_pos = -60.50001;
            
            println!("Y PITCH: {:?}", camera_pitch);
            println!("ROTATION {}", cam);

            // camera looking up
            if camera_pitch > 0.66 {
                println!("Camera is more more more more looking up");
                new_y_pos += camera_pitch * 310.0; 
            } 
            else if camera_pitch > 0.6 {
                println!("Camera is more more more looking up");
                new_y_pos += camera_pitch * 290.0; 
            } 
            else if camera_pitch > 0.2 {
                println!("Camera is more more looking up");
                new_y_pos += camera_pitch * 80.0; 
            } 
            else if camera_pitch > 0.11 {
                println!("Camera is more looking up");
                new_y_pos += camera_pitch * 100.0; 
            } 
            else if camera_pitch > 0.004 {
                println!("Camera is looking up");
                new_y_pos += camera_pitch * 120.0; 
            }

            else if camera_pitch < -0.2 {
                println!("Camera is more looking down");
                new_y_pos += camera_pitch * 40.0;
                new_z_pos += camera_pitch.abs() * 80.0;
         
            }
          
            else if camera_pitch < -0.1 {
                println!("Camera is more looking down");
                new_y_pos += camera_pitch * 30.0;
                new_z_pos += camera_pitch.abs() * 80.0;
         
            }
              // camera looking down
              else if camera_pitch < -0.05 {
                println!("Camera is looking down");
                new_y_pos += camera_pitch * 10.0;
                new_z_pos += camera_pitch.abs() * 100.0;
         
            } 
              else {
                println!("Camera is looking forward horizontally");
            }
            
            transform.translation = Vec3::new(48.0, new_y_pos, new_z_pos);

            // rotation
            let pitch_rotation = Quat::from_rotation_x(camera_pitch * 1.5);
            transform.rotation = pitch_rotation;
        }
       }
    }

    // Update visibility based on the muzzle flash timer
    for (_, mut muzzle_flash, mut visibility, _) in muzzle_flash_query.iter_mut() {
       if muzzle_flash.is_active {
        muzzle_flash.frames_visible += 1;
        if muzzle_flash.timer.tick(time.delta()).just_finished() || muzzle_flash.frames_visible >= 10 {
       
            *visibility = Visibility::Hidden;
            muzzle_flash.is_active = false;
            muzzle_flash.frames_visible = 0;
        }
       }
    }
}