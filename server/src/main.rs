use bevy_app::{App, ScheduleRunnerPlugin};
use bevy_core::CorePlugin;
use bevy_log::{info, LogPlugin};

use naia_bevy_server::{Plugin as ServerPlugin, ServerConfig, Stage};
use network_shared::{protocol::Protocol, shared_config, Channels};
use systems::{init, tick, events};

mod resources;
mod systems;

fn main() {
    info!("Naia Server starting up");
    
    App::default()
        .add_plugin(CorePlugin::default())
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(LogPlugin::default())
        .add_plugin(ServerPlugin::<Protocol, Channels>::new(
            ServerConfig::default(),
            shared_config()
        ))
        .add_startup_system(init)
        .add_system_to_stage(Stage::ReceiveEvents, events::authorization_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::connection_event)
        .add_system_to_stage(Stage::Tick, tick)
        .run();
}
