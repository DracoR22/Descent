use bevy::prelude::*;
use bevy::window::CursorGrabMode;

use crate::game::state::GameState;

pub fn setup_window(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}

pub fn toggle_cursor(mut windows: Query<&mut Window>, keyboard_input: Res<ButtonInput<KeyCode>>, state: Res<State<GameState>>) {
    let mut window = windows.single_mut();
    
    if keyboard_input.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = match window.cursor.grab_mode {
            CursorGrabMode::Locked => CursorGrabMode::None,
            _ => CursorGrabMode::Locked,
        };
        window.cursor.visible = !window.cursor.visible;
    }
}

pub fn handle_pause_state(
    mut windows: Query<&mut Window>,
    state: Res<State<GameState>>,
    mut last_state: Local<Option<GameState>>,
) {
    let mut window = windows.single_mut();

    // Check for state changes to avoid continuous toggling
    if last_state.as_ref() != Some(state.get()) {
        if *state.get() == GameState::Paused {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        } else {
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
        }
    }

    *last_state = Some(state.get().clone());
}