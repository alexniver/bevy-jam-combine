use bevy::prelude::*;

use crate::common::WINDOW_SIZE;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ui);
    }
}

#[derive(Component)]
pub struct UI(pub Vec3);

#[derive(Component)]
pub struct UIHelper; // helper panel
#[derive(Component)]
pub struct UIInfo; // info panel
#[derive(Component)]
pub struct UIBuild; // build panel
#[derive(Component)]
pub struct UILog; // build panel

#[derive(Component)]
pub struct InfoDate;
#[derive(Component)]
pub struct InfoSelectorAxis;
#[derive(Component)]
pub struct InfoCash;

fn ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_size = 15.;

    let text_style = TextStyle {
        font: asset_server.load("fonts/PixelEmulator-xq08.ttf"),
        font_size,
        color: Color::DARK_GRAY,
    };
    // root node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        // info panel
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(300.0), Val::Px(200.0)),
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            top: Val::Px(0.0),
                            right: Val::Px(0.0),
                            ..default()
                        },
                        ..default()
                    },
                    color: Color::rgba(0.4, 0.4, 1.0, 0.3).into(),
                    ..default()
                })
                .with_children(|parent| {
                    let mut top = 0.;

                    parent
                        .spawn_bundle(
                            TextBundle::from_section(format!("Date: 2022-1-1"), text_style.clone())
                                .with_style(Style {
                                    size: Size::new(Val::Auto, Val::Auto),
                                    position_type: PositionType::Absolute,
                                    position: UiRect {
                                        top: Val::Px(top),
                                        left: Val::Px(0.0),
                                        ..default()
                                    },
                                    ..default()
                                }),
                        )
                        .insert(InfoDate);

                    top += font_size;

                    parent
                        .spawn_bundle(
                            TextBundle::from_section(format!("Axis: "), text_style.clone())
                                .with_style(Style {
                                    size: Size::new(Val::Auto, Val::Auto),
                                    position_type: PositionType::Absolute,
                                    position: UiRect {
                                        top: Val::Px(top),
                                        left: Val::Px(0.0),
                                        ..default()
                                    },
                                    ..default()
                                }),
                        )
                        .insert(InfoSelectorAxis);

                    top += font_size;

                    parent
                        .spawn_bundle(
                            TextBundle::from_section(format!("Cash: 100"), text_style.clone())
                                .with_style(Style {
                                    size: Size::new(Val::Auto, Val::Auto),
                                    position_type: PositionType::Absolute,
                                    position: UiRect {
                                        top: Val::Px(top),
                                        left: Val::Px(0.0),
                                        ..default()
                                    },
                                    ..default()
                                }),
                        )
                        .insert(InfoCash);
                })
                .insert(UIInfo);
        })
        // help panel
        .with_children(|parent| {
            let help_width = 600.0;
            let help_height = 600.0;
            let helper_msg_arr = [
                "1. use 'A' 'S' 'D' 'W' move camera, 'C' reset",
                "2. use 'Z' 'X' zoom camera",
                "3. use 'H' 'J' 'K' 'L' move cursor",
                "4. use 'V' start 'region selection' mode",
                "5. use 'I' start 'build' mode",
                "6. use 'Esc' to escape 'mode'",
                "7. sand will hurt grass, '30 hp' one 'neighbour sand' one day",
                "8. grass has '100 hp'",
                "9. reinforce the sand, keeps sand no damage for 6 month",
                "10. plant tree can turn sand to grass",
                // "11. sapling will be damaged by sand too, but they have '20,000 hp'",
                // "12. sapling grow up to tree after one year, tree have '100,000 hp'",
                // "13. worm eat tree, '1000 hp' one day!",
                // "14. chicken eat worm",
                // "15. solar panels generate electric, earn 10 cash one day",
                // "16. solar panel condensate every morning, make grass grow fast",
                // "17. when grass too high, solar panel will lose 70% efficiency",
                // "17. sheep eat grass",
                // "18. chicken need 4 month to grow up, worth 100 cash",
                // "19. sheep need 10 month to grow up, worth 1200 cash",
                // "20. chicken and sheep will auto sold after 1 month they grow up",
                "11. the tree called 'Elaeagnus angustifolia'",
                "12. this game inspared by 'Great Green Wall of China'",
                "13. press 'F1' close/open help",
            ];
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(help_width), Val::Px(help_height)),
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px((WINDOW_SIZE.0 - help_width) / 2. - 100.),
                            bottom: Val::Px((WINDOW_SIZE.1 - help_height) / 2.),
                            ..default()
                        },
                        ..default()
                    },
                    color: Color::rgba(0.4, 0.4, 1.0, 0.3).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(
                        TextBundle::from_section("Pause", text_style.clone()).with_style(Style {
                            size: Size::new(Val::Auto, Val::Auto),
                            position_type: PositionType::Absolute,
                            position: UiRect {
                                top: Val::Px(0.),
                                left: Val::Px(help_width / 2. - 20.),
                                ..default()
                            },
                            ..default()
                        }),
                    );
                    let mut top = 30.;

                    for msg in helper_msg_arr {
                        parent.spawn_bundle(
                            TextBundle::from_section(msg, text_style.clone()).with_style(Style {
                                size: Size::new(Val::Auto, Val::Auto),
                                position_type: PositionType::Absolute,
                                position: UiRect {
                                    top: Val::Px(top),
                                    left: Val::Px(0.0),
                                    ..default()
                                },
                                ..default()
                            }),
                        );

                        top += font_size;
                    }
                })
                .insert(UIHelper);
        })
        // build panel
        .with_children(|parent| {
            let build_width = 300.;
            let build_height = 300.;
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(build_width), Val::Px(build_height)),
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            right: Val::Px(0.),
                            bottom: Val::Px(0.),
                            ..default()
                        },
                        ..default()
                    },
                    color: Color::rgba(0.4, 0.4, 1.0, 0.3).into(),
                    visibility: Visibility { is_visible: false },
                    ..default()
                })
                .insert(UIBuild);
        });
}
