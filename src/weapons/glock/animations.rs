use std::time::Duration;

use bevy::prelude::*;

use crate::{game::link_animations::{AnimationEntityLink, MultipleAnimationEntityLinks}, player::components::Player, weapons::resources::{CurrentWeapon, GlockAnimations}};

#[derive(PartialEq, Clone, Copy)]
pub enum GlockAnimationsList {
    IDLE,
    WALK,
    SHOOT,
    RELOADFAST,
    RELOADFULL
}

impl Default for GlockAnimationsList {
    fn default() -> Self {
        Self::IDLE
    }
}

impl From<&KeyCode> for GlockAnimationsList {
    fn from(key_code: &KeyCode) -> Self {
        match key_code {
            KeyCode::KeyW => GlockAnimationsList::WALK,
            KeyCode::KeyA => GlockAnimationsList::WALK,
            KeyCode::KeyS => GlockAnimationsList::WALK,
            KeyCode::KeyD => GlockAnimationsList::WALK,
            KeyCode::KeyR => GlockAnimationsList::RELOADFULL,

            _ => GlockAnimationsList::IDLE,
        }
    }
}

pub fn setup_glock_animations(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GlockAnimations(vec![
        asset_server.load("animations/glock.glb#Animation0"), 
        asset_server.load("animations/glock.glb#Animation1"),
        asset_server.load("animations/glock.glb#Animation2"),
        asset_server.load("animations/glock.glb#Animation3"),
        asset_server.load("animations/glock.glb#Animation4"),
  ]));
}

pub fn load_glock_animation(
    animations: Res<GlockAnimations>,
    mut players_query: Query<&mut AnimationPlayer>,
    mut current_animation: Local<GlockAnimationsList>,
    mut player_character_query: Query<(&Player, &MultipleAnimationEntityLinks)>, // Use AnimationEntityLinks
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    state: Res<State<CurrentWeapon>>
) {
    match state.get() {
        CurrentWeapon::Glock => {
            for (player_entity, animation_entity_links) in player_character_query.iter_mut() {
                for &animation_entity in &animation_entity_links.0 { // Iterate through all linked entities
                    if let Ok(mut animation_player) = players_query.get_mut(animation_entity) {
                        keyboard_input.get_just_pressed().into_iter().for_each(|key_code| {
                            *current_animation = GlockAnimationsList::from(key_code);
                        });
            
                        if mouse_input.just_pressed(MouseButton::Left) {
                            *current_animation = GlockAnimationsList::SHOOT;
                        }
            
                        if mouse_input.just_released(MouseButton::Left) {
                            // Stop looping or switch animation when left-click is released
                            *current_animation = GlockAnimationsList::IDLE; // Reset or change animation on release
                        }
            
                        if keyboard_input.just_released(KeyCode::KeyW)
                            || keyboard_input.just_released(KeyCode::KeyA)
                            || keyboard_input.just_released(KeyCode::KeyS)
                            || keyboard_input.just_released(KeyCode::KeyD) {
                            *current_animation = GlockAnimationsList::IDLE;
                        }
            
                        if *current_animation != GlockAnimationsList::IDLE && animation_player.is_finished() {
                            *current_animation = GlockAnimationsList::IDLE;
                        }
            
                        let animation: &mut AnimationPlayer = animation_player.play_with_transition(
                            animations.0[*current_animation as usize].clone_weak(), 
                            Duration::from_millis(100), // transition duration
                        );
            
                        if *current_animation == GlockAnimationsList::WALK || *current_animation == GlockAnimationsList::IDLE {
                            animation.repeat();
                        }
            
                        if *current_animation == GlockAnimationsList::SHOOT {
                            animation.repeat();
                            animation.set_speed(1.0); 
                        }
                    }
                }
            }
        }
        _ => ()
    }
}
