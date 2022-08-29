use bevy::prelude::*;

pub struct LogicTreePlugin;
impl Plugin for LogicTreePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(sold_event);
    }
}

fn sold_event() {}
