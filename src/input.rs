use crossterm::event::{KeyCode, KeyEvent};

use crate::game::GameState;

pub fn handle_key(state: &mut GameState, key: KeyEvent) {
    match key.code {
        KeyCode::Char('p') => {
            state.toggle_pause();
            return;
        }
        KeyCode::Char('r') if state.game_over => {
            state.reset();
            return;
        }
        _ => {}
    }

    if state.game_over || state.paused {
        return;
    }

    match key.code {
        KeyCode::Left | KeyCode::Char('h') => state.move_horizontal(-1),
        KeyCode::Right | KeyCode::Char('l') => state.move_horizontal(1),
        KeyCode::Down | KeyCode::Char('j') => state.soft_drop(),
        KeyCode::Up | KeyCode::Char('k') => state.rotate(),
        KeyCode::Char(c) if c == ' ' => state.hard_drop(),
        KeyCode::Enter => state.hard_drop(),
        _ => {}
    }
}
