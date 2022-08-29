use std::time::Duration;

use crate::{gamestate_plugin::GameState, ui_plugin::InfoDate};
use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct GameDatePlugin;

impl Plugin for GameDatePlugin {
    fn build(&self, app: &mut App) {
        // stage for anything we want to do on a fixed timestep
        let mut fixedupdate = SystemStage::parallel();
        fixedupdate.add_system(
            date_system
                // only do it when playing
                .run_in_state(GameState::Playing),
        );

        app.insert_resource(GameDate::default())
            .add_event::<GameDateOneDayEvent>()
            .add_stage_before(
                CoreStage::Update,
                "FixedUpdate",
                FixedTimestepStage::from_stage(Duration::from_millis(5 * 1000), fixedupdate),
            )
            .add_system(date_ui);
    }
}

pub struct GameDate {
    pub year: u16,
    pub month: u16,
    pub day: u16,
}

impl GameDate {
    pub fn add_one_day(&mut self) {
        self.day += 1;

        if self.day > 30 {
            self.day = 1;
            self.month += 1;

            if self.month > 12 {
                self.month = 1;
                self.year += 1;
            }
        }
    }
}

impl Default for GameDate {
    fn default() -> Self {
        Self {
            year: 2022,
            month: 1,
            day: 1,
        }
    }
}

#[derive(Default)]
pub struct GameDateOneDayEvent; // 经历1天 event

fn date_system(
    mut game_date: ResMut<GameDate>,
    mut one_day_event: EventWriter<GameDateOneDayEvent>,
) {
    game_date.add_one_day();
    one_day_event.send_default();
}

fn date_ui(
    one_day_event: EventReader<GameDateOneDayEvent>,
    date: Res<GameDate>,
    mut text_query: Query<&mut Text, With<InfoDate>>,
) {
    if one_day_event.is_empty() {
        return;
    }
    let mut text = text_query.single_mut();
    text.sections[0].value = format!("Date: {}-{}-{}", date.year, date.month, date.day);
}
