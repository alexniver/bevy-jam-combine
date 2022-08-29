use bevy::prelude::Plugin;

struct CashPlugin;
impl Plugin for CashPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Cash(0)).add_event::<CashChangeEvent>();
    }
}

pub struct Cash(pub i32);

pub struct CashChangeEvent(pub i32); // change num
