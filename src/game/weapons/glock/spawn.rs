use bevy::prelude::*;

use crate::{common::{entities::WeaponType, states::CurrentWeapon}, game::{player::components::{Player, PlayerFirstPersonCamera}, weapons::{glock::{components::{GlockComponent, HasGlock}, GLOCK_MAGAZINE_SIZE, MAX_GLOCK_AMMO}, HasWeapon}}};

pub const HIDE_OFFSET: f32 = -1000.0;

pub fn update_gun_rotation(
    camera_query: Query<&Transform, With<PlayerFirstPersonCamera>>,
    mut gun_query: Query<(&mut Transform, Option<&Parent>), (With<GlockComponent>, Without<PlayerFirstPersonCamera>)>,
) {
 
   
        if let Ok(camera_transform) = camera_query.get_single() {
            if let Ok((mut gun_transform, maybe_parent)) = gun_query.get_single_mut() {
          
                if maybe_parent.is_some() {
                    let gun_rotation = camera_transform.rotation * Quat::from_rotation_y(std::f32::consts::PI);
    
                    gun_transform.rotation = gun_rotation;
        
                    let forward_vec = camera_transform.forward();
                    let camera_pitch = forward_vec.y; 
        
                    let dynamic_z_offset = if camera_pitch >= 0.0 {
                        2.9 - (camera_pitch * -5.2)  // Move closer when looking up
                    } else {
                        2.9 + (camera_pitch.abs() * -5.2)  // Move further away when looking down
                    };
        
                    // Adjust the gun's position relative to the camera
                    gun_transform.translation = camera_transform.translation
                        + camera_transform.forward() * dynamic_z_offset // Move it forward 
                        + camera_transform.right() * 0.15 // Move it to the right 
                        - camera_transform.up() * 0.1 // Move it down 
                        + Vec3::new(0.0, -4.3, 0.0); // Additional downward offset
                

               
              }
            }
        }
    
    
   
}
