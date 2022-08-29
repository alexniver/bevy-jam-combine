use bevy::prelude::*;

pub struct GameModePlugin;

impl Plugin for GameModePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameModeStack::default())
            .add_system(game_mode_exchange);
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameMode {
    Selector,
    TileBuild,
}

pub struct GameModeStack {
    stack: Vec<GameMode>,
}

impl GameModeStack {
    pub fn push(&mut self, game_mode: GameMode) {
        self.stack.push(game_mode);
    }
    pub fn pop(&mut self) -> Option<GameMode> {
        self.stack.pop()
    }
    pub fn peek(&self) -> Option<&GameMode> {
        self.stack.last()
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }
}

impl Default for GameModeStack {
    fn default() -> Self {
        let mut res = Self {
            stack: Default::default(),
        };
        res.push(GameMode::Selector);
        res
    }
}

fn game_mode_exchange(
    keyboard_res: Res<Input<KeyCode>>,
    mut game_mode_stack: ResMut<GameModeStack>,
) {
    let game_mode = game_mode_stack.peek();
    if game_mode.is_some() {
        match game_mode.unwrap() {
            GameMode::Selector => {
                if keyboard_res.just_pressed(KeyCode::I) {
                    game_mode_stack.push(GameMode::TileBuild);
                }
            }

            GameMode::TileBuild => {
                if keyboard_res.just_pressed(KeyCode::Escape) {
                    game_mode_stack.pop();
                }
            }
        }
    }
}
