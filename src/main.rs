use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy_ecs_tilemap::prelude::*;
use desert_terminator::build_plugin::BuildPlugin;
use desert_terminator::camera_plugin::CameraPlugin;
use desert_terminator::common::*;
use desert_terminator::gamedate_plugin::GameDatePlugin;
use desert_terminator::gamemode_plugin::*;
use desert_terminator::gamestate_plugin::GameStatePlugin;
use desert_terminator::helper_plugin::HelperPlugin;
use desert_terminator::logic_chick_plugin::LogicChickPlugin;
use desert_terminator::logic_chicken_plugin::LogicChickenPlugin;
use desert_terminator::logic_grass_plugin::LogicGrassPlugin;
use desert_terminator::logic_sand_plugin::LogicSandPlugin;
use desert_terminator::logic_selector_plugin::LogicSelectorPlugin;
use desert_terminator::logic_tree_little_plugin::LogicTreeLittlePlugin;
use desert_terminator::logic_tree_plugin::LogicTreePlugin;
use desert_terminator::spawn_tilemap_plugin::SpawnPlugin;
use desert_terminator::tile_chick_plugin::TileChickPlugin;
use desert_terminator::tile_chicken_plugin::TileChickenPlugin;
use desert_terminator::tile_grass_plugin::TileGrassPlugin;
use desert_terminator::tile_grass_solid_plugin::TileGrassSolidPlugin;
use desert_terminator::tile_lamb_plugin::TileLambPlugin;
use desert_terminator::tile_sand_plugin::TileSandPlugin;
use desert_terminator::tile_sand_solid_plugin::TileSandSolidPlugin;
use desert_terminator::tile_selector_plugin::TileSelectorPlugin;
use desert_terminator::tile_sheep_plugin::TileSheepPlugin;
use desert_terminator::tile_solar_panel_grass_plugin::TileSolarPanelGrassPlugin;
use desert_terminator::tile_solar_panel_plugin::TileSolarPanelPlugin;
use desert_terminator::tile_tree_little_plugin::TileTreeLittlePlugin;
use desert_terminator::tile_tree_plugin::TileTreePlugin;
use desert_terminator::tile_worm_plugin::TileWormPlugin;
use desert_terminator::ui_plugin::UIPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: WINDOW_SIZE.0,
            height: WINDOW_SIZE.1,
            title: String::from("Desert Timinator"),
            ..Default::default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(TileSandPlugin)
        .add_plugin(TileSandSolidPlugin)
        .add_plugin(TileGrassPlugin)
        .add_plugin(TileGrassSolidPlugin)
        .add_plugin(TileTreePlugin)
        .add_plugin(TileTreeLittlePlugin)
        .add_plugin(TileSolarPanelPlugin)
        .add_plugin(TileSolarPanelGrassPlugin)
        .add_plugin(TileChickPlugin)
        .add_plugin(TileChickenPlugin)
        .add_plugin(TileLambPlugin)
        .add_plugin(TileSheepPlugin)
        .add_plugin(TileWormPlugin)
        .add_plugin(TileSelectorPlugin)
        .add_plugin(GameStatePlugin)
        .add_plugin(GameModePlugin)
        .add_plugin(GameDatePlugin)
        .add_plugin(UIPlugin)
        .add_plugin(HelperPlugin)
        .add_plugin(LogicSelectorPlugin)
        .add_plugin(LogicSandPlugin)
        .add_plugin(LogicGrassPlugin)
        .add_plugin(LogicChickPlugin)
        .add_plugin(LogicChickenPlugin)
        .add_plugin(LogicTreeLittlePlugin)
        .add_plugin(LogicTreePlugin)
        .add_plugin(BuildPlugin)
        // .add_plugin(SolarPanelPlugin)
        .add_plugin(SpawnPlugin)
        .run();
}

/*
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: WINDOW_SIZE.0,
            height: WINDOW_SIZE.1,
            title: String::from("Desert Timinator"),
            ..Default::default()
        })
        .add_startup_system(startup_1)
        .add_startup_system(startup_2.after(startup_1))
        // .add_startup_system_to_stage(StartupStage::PostStartup, startup_2)
        .run();
}

#[derive(Component)]
struct Comp1;

fn startup_1(mut commands: Commands) {
    commands.spawn().insert(Comp1);
}

fn startup_2(mut commands: Commands, q: Query<Entity, With<Comp1>>) {
    let e = q.single();
}

 */
