use bevy::prelude::*;
use iyes_loopless::state::CurrentState;

use crate::{gamestate_plugin::GameState, ui_plugin::UIHelper};

pub struct HelperPlugin;

impl Plugin for HelperPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(helper_ui);
    }
}

fn helper_ui(state: Res<CurrentState<GameState>>, mut q: Query<&mut Visibility, With<UIHelper>>) {
    if state.is_changed() {
        let game_state = state.into_inner().0;
        let mut v = q.single_mut();
        if game_state == GameState::Playing {
            v.is_visible = false;
        } else if game_state == GameState::Pause {
            v.is_visible = true;
        }
    }
}
