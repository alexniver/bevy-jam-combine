use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameState::Pause)
            .add_system(gamestate_set);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Pause,
    Playing,
    GameOver,
}

fn gamestate_set(
    mut commands: Commands,
    state: Res<CurrentState<GameState>>,
    keyboard: Res<Input<KeyCode>>,
) {
    let game_state = state.into_inner().0;
    if keyboard.just_pressed(KeyCode::F1) {
        // pause
        if game_state != GameState::GameOver {
            if game_state == GameState::Playing {
                // game_state = GameState::Pause;
                commands.insert_resource(NextState(GameState::Pause));
            } else {
                // game_state = GameState::Playing;
                commands.insert_resource(NextState(GameState::Playing));
            }
        }
    }
}
