use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    gamemode_plugin::{GameMode, GameModeStack},
    gamestate_plugin::GameState,
    logic_selector_plugin::{Selector, SelectorMoveEvent},
    ui_plugin::UIBuild,
};

pub struct BuildPlugin;

impl Plugin for BuildPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BuildIndex(0))
            .add_event::<BuildSolidEvent>()
            .add_event::<BuildTreeEvent>()
            .add_event::<BuildChickEvent>()
            .add_event::<BuildLambEvent>()
            .add_event::<BuildSolarPanelEvent>()
            .add_event::<RecoveryEvent>()
            .add_event::<SoldAllEvent>()
            .add_system(build_trigger.run_in_state(GameState::Playing));
    }
}

pub struct BuildSolidEvent(pub u32, pub u32);
pub struct BuildTreeEvent(pub u32, pub u32);
pub struct BuildChickEvent(pub u32, pub u32);
pub struct BuildLambEvent(pub u32, pub u32);
pub struct BuildSolarPanelEvent(pub u32, pub u32);
pub struct RecoveryEvent(pub u32, pub u32);
pub struct SoldAllEvent(pub u32, pub u32);

// build index, 0 means none
struct BuildIndex(u8);

const BUILD_LIST: [&str; 2] = [
    "1. reinforce sand [5]",
    "2. plant tree [10]",
    // "3. a chick [5]",
    // "4. a lamb [50]",
    // "5. solar panel [1000]",
    // "4. recovery tree[100]/solar panel[500]",
    // "6. recovery tree[100]/solar panel[500]",
    // "5. sold all chicken[50]",
    // "7. sold all chicken[50]/sheep[1000]",
    // "8. ",
    // "9. ",
];

fn build_trigger(
    mut commands: Commands,
    keyboard_res: Res<Input<KeyCode>>,
    mut game_mode_stack: ResMut<GameModeStack>,
    mut q_buildui: Query<(Entity, &mut Visibility), With<UIBuild>>,
    mut build_index: ResMut<BuildIndex>,
    asset_server: Res<AssetServer>,
    mut selector: ResMut<Selector>,
    mut selector_move_event: EventWriter<SelectorMoveEvent>,
    mut build_solid_event: EventWriter<BuildSolidEvent>,
    mut build_tree_event: EventWriter<BuildTreeEvent>,
    mut build_chick_event: EventWriter<BuildChickEvent>,
    mut build_lamb_event: EventWriter<BuildLambEvent>,
    mut build_solar_panel_event: EventWriter<BuildSolarPanelEvent>,
    mut recovery_event: EventWriter<RecoveryEvent>,
    mut sold_all_event: EventWriter<SoldAllEvent>,
) {
    if let Some(game_mode) = game_mode_stack.peek() {
        let (buildui_entity, mut visibility) = q_buildui.single_mut();
        if *game_mode != GameMode::TileBuild {
            visibility.is_visible = false;
            return;
        }

        visibility.is_visible = true;

        // keyboard event
        if keyboard_res.just_pressed(KeyCode::Key1) {
            build_index.0 = 1;
        } else if keyboard_res.just_pressed(KeyCode::Key2) {
            build_index.0 = 2;
        } else if keyboard_res.just_pressed(KeyCode::Key3) {
            build_index.0 = 3;
        } else if keyboard_res.just_pressed(KeyCode::Key4) {
            build_index.0 = 4;
        } else if keyboard_res.just_pressed(KeyCode::Key5) {
            build_index.0 = 5;
        } else if keyboard_res.just_pressed(KeyCode::Key6) {
            // build_index.0 = 6;
        } else if keyboard_res.just_pressed(KeyCode::Key7) {
            // build_index.0 = 7;
        } else if keyboard_res.just_pressed(KeyCode::Key8) {
            // build_index.0 = 8;
        } else if keyboard_res.just_pressed(KeyCode::Key9) {
            // build_index.0 = 9;
        }

        // 建造
        if keyboard_res.just_pressed(KeyCode::Return) {
            let min_x = u32::min(selector.pos1.x, selector.pos2.x);
            let max_x = u32::max(selector.pos1.x, selector.pos2.x);

            let min_y = u32::min(selector.pos1.y, selector.pos2.y);
            let max_y = u32::max(selector.pos1.y, selector.pos2.y);
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    if build_index.0 == 1 {
                        build_solid_event.send(BuildSolidEvent(x, y));
                    } else if build_index.0 == 2 {
                        build_tree_event.send(BuildTreeEvent(x, y));
                    } else if build_index.0 == 3 {
                        build_chick_event.send(BuildChickEvent(x, y));
                    } else if build_index.0 == 4 {
                        build_lamb_event.send(BuildLambEvent(x, y));
                    } else if build_index.0 == 5 {
                        build_solar_panel_event.send(BuildSolarPanelEvent(x, y));
                    } else if build_index.0 == 6 {
                        recovery_event.send(RecoveryEvent(x, y));
                    } else if build_index.0 == 7 {
                        sold_all_event.send(SoldAllEvent(x, y));
                    }
                }
            }

            if build_index.0 != 0 {
                build_index.0 = 0;

                selector.is_area = false;
                selector.pos1 = selector.pos2;
                selector_move_event.send_default();

                game_mode_stack.pop();
                return;
            }
        }

        // remove all children
        let mut buildui_entity_commands = commands.entity(buildui_entity);
        buildui_entity_commands.despawn_descendants();

        let font_size = 15.;
        let text_style_normal = TextStyle {
            font: asset_server.load("fonts/PixelEmulator-xq08.ttf"),
            font_size,
            color: Color::DARK_GRAY,
        };
        let text_style_select = TextStyle {
            font: asset_server.load("fonts/PixelEmulator-xq08.ttf"),
            font_size,
            color: Color::ORANGE_RED,
        };

        buildui_entity_commands.with_children(|parent| {
            let mut top = 0.;
            for (idx, build_str) in BUILD_LIST.iter().enumerate() {
                if idx > 5 {
                    break;
                }
                let mut text_style = text_style_normal.clone();
                if idx + 1 == build_index.0 as usize {
                    text_style = text_style_select.clone();
                }
                parent.spawn_bundle(TextBundle::from_section(*build_str, text_style).with_style(
                    Style {
                        size: Size::new(Val::Auto, Val::Auto),
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            top: Val::Px(top),
                            left: Val::Px(0.0),
                            ..default()
                        },
                        ..default()
                    },
                ));

                top += font_size;
            }

            parent.spawn_bundle(
                TextBundle::from_section("press 'Enger' to build", text_style_normal).with_style(
                    Style {
                        size: Size::new(Val::Auto, Val::Auto),
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            bottom: Val::Px(0.),
                            left: Val::Px(0.0),
                            ..default()
                        },
                        ..default()
                    },
                ),
            );
        });

        // clear
    }
}
