use crate::common::*;
use crate::gamemode_plugin::*;
use crate::gamestate_plugin::GameState;
use crate::tile_selector_plugin::DespawnSelectorTileEvent;
use crate::tile_selector_plugin::SpawnSelectorTileEvent;
use crate::ui_plugin::*;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct LogicSelectorPlugin;
impl Plugin for LogicSelectorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Selector::default())
            .add_event::<SelectorMoveEvent>()
            .add_startup_system(setup)
            .add_system(selector_move.run_in_state(GameState::Playing))
            .add_system(selector_move_event);
    }
}

pub struct Selector {
    pub is_area: bool,
    pub pos1: UVec2,
    pub pos2: UVec2,
}

impl Default for Selector {
    fn default() -> Self {
        let pos = UVec2::new(MAP_SIZE / 2, MAP_SIZE / 2);
        Self {
            is_area: false,
            pos1: pos,
            pos2: pos,
        }
    }
}

#[derive(Default)]
pub struct SelectorMoveEvent;

fn setup(mut selector_move_event: EventWriter<SelectorMoveEvent>) {
    selector_move_event.send_default();
}

fn selector_move(
    keyboard_res: Res<Input<KeyCode>>,
    game_mode_stack: ResMut<GameModeStack>,
    mut selector: ResMut<Selector>,
    mut selector_move_event: EventWriter<SelectorMoveEvent>,
) {
    let game_mode = game_mode_stack.peek().unwrap();
    if *game_mode == GameMode::TileBuild {
        return;
    }
    let mut is_press = false;
    if keyboard_res.just_pressed(KeyCode::H) {
        if selector.pos2.x > 0 {
            selector.pos2.x -= 1;
            is_press = true;
        }
    } else if keyboard_res.just_pressed(KeyCode::J) {
        if selector.pos2.y > 0 {
            selector.pos2.y -= 1;
            is_press = true;
        }
    } else if keyboard_res.just_pressed(KeyCode::K) {
        if selector.pos2.y < MAP_SIZE - 1 {
            selector.pos2.y += 1;
            is_press = true;
        }
    } else if keyboard_res.just_pressed(KeyCode::L) {
        if selector.pos2.x < MAP_SIZE - 1 {
            selector.pos2.x += 1;
            is_press = true;
        }
    }

    if keyboard_res.just_pressed(KeyCode::V) {
        selector.is_area = !selector.is_area;
        is_press = true;
    }

    if is_press {
        if *game_mode == GameMode::Selector {
            if !selector.is_area {
                selector.pos1 = selector.pos2;
            }
        }
        selector_move_event.send_default();
    }
}

fn selector_move_event(
    selector: Res<Selector>,
    game_mode_stack: Res<GameModeStack>,
    selector_move_event: EventReader<SelectorMoveEvent>,
    mut selector_spawn_event: EventWriter<SpawnSelectorTileEvent>,
    mut selector_despawn_event: EventWriter<DespawnSelectorTileEvent>,
    mut query_text: Query<&mut Text, With<InfoSelectorAxis>>,
) {
    if selector_move_event.is_empty() {
        return;
    }

    selector_move_event.clear();
    let min_x = u32::min(selector.pos1.x, selector.pos2.x);
    let max_x = u32::max(selector.pos1.x, selector.pos2.x);

    let min_y = u32::min(selector.pos1.y, selector.pos2.y);
    let max_y = u32::max(selector.pos1.y, selector.pos2.y);

    let mut clear_tilemap = || {
        for x in 0..MAP_SIZE {
            for y in 0..MAP_SIZE {
                if x < min_x || x > max_x || y < min_y || y > max_y {
                    selector_despawn_event.send(DespawnSelectorTileEvent(x, y));
                }
            }
        }
    };

    let game_mode = game_mode_stack.peek();
    if game_mode.is_some() {
        match game_mode.unwrap() {
            GameMode::Selector => {
                // 清理旧tilemap
                clear_tilemap();

                // 根据pos1 和pos2的区域画区域的tile
                for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        selector_spawn_event.send(SpawnSelectorTileEvent(x, y));
                    }
                }
            }
            GameMode::TileBuild => { /* do nothing */ }
        }

        query_text.single_mut().sections[0].value = format!(
            "Axis: {}, {}-{}, {}",
            selector.pos1.x, selector.pos1.y, selector.pos2.x, selector.pos2.y,
        );
    }
}
