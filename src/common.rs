use bevy::prelude::*;

pub const WINDOW_SIZE: (f32, f32) = (1024., 768.);

pub const TILE_SIZE: f32 = 32.;
pub const MAP_SIZE: u32 = 128;

#[derive(Component)]
pub struct LayerSand;
#[derive(Component)]
pub struct LayerSandSolid;
#[derive(Component)]
pub struct LayerGrass;
#[derive(Component)]
pub struct LayerGrassSolid;
#[derive(Component)]
pub struct LayerTreeLittle;
#[derive(Component)]
pub struct LayerTree;
#[derive(Component)]
pub struct LayerSolarPanel;
#[derive(Component)]
pub struct LayerSolarPanelGrass;
#[derive(Component)]
pub struct LayerChick;
#[derive(Component)]
pub struct LayerChicken;
#[derive(Component)]
pub struct LayerLamb;
#[derive(Component)]
pub struct LayerSheep;
#[derive(Component)]
pub struct LayerWorm;
#[derive(Component)]
pub struct LayerSelector;

#[derive(Component)]
pub struct TileSand;
#[derive(Component)]
pub struct TileSandSolid(pub u32); // days left
#[derive(Component)]
pub struct TileGrass(pub i32); // hp
#[derive(Component)]
pub struct TileGrassSolid(pub u32); // days left
#[derive(Component)]
pub struct TileTreeLittle(pub u32, pub i32); // growup left days, hp
#[derive(Component)]
pub struct TileTree(pub u32); // hp
#[derive(Component)]
pub struct TileSolarPanel;
#[derive(Component)]
pub struct TileSolarPanelGrass;
#[derive(Component)]
pub struct TileChick;
#[derive(Component)]
pub struct TileChicken;
#[derive(Component)]
pub struct TileLamb;
#[derive(Component)]
pub struct TileSheep;
#[derive(Component)]
pub struct TileWorm;
#[derive(Component)]
pub struct TileSelector;

pub const LAYER_Z_SAND: f32 = 0.01;
pub const LAYER_Z_SAND_SOLID: f32 = 0.02;
pub const LAYER_Z_GRASS: f32 = 0.03;
pub const LAYER_Z_GRASS_SOLID: f32 = 0.04;
pub const LAYER_Z_TREE_LITTLE: f32 = 0.05;
pub const LAYER_Z_TREE: f32 = 0.06;
pub const LAYER_Z_SOLAR_PANEL: f32 = 0.07;
pub const LAYER_Z_SOLAR_PANEL_GRASS: f32 = 0.08;
pub const LAYER_Z_CHICK: f32 = 0.09;
pub const LAYER_Z_CHICKEN: f32 = 0.10;
pub const LAYER_Z_LAMP: f32 = 0.11;
pub const LAYER_Z_SHEEP: f32 = 0.12;
pub const LAYER_Z_WORM: f32 = 0.13;
pub const LAYER_Z_SELECTOR: f32 = 0.14;

pub const SAND_SOLID_DAYS: u32 = 6 * 30;
pub const GRASS_SOLID_DAYS: u32 = 6 * 30;

pub const TREE_HP: u32 = 100000;
// pub const TREE_LITTLE_GROW_DAYS: u32 = 5;
pub const TREE_LITTLE_GROW_DAYS: u32 = 8 * 30;
pub const TREE_LITTLE_HP: i32 = 180;
