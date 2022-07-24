use bevy_ecs::prelude::Commands;
use bevy_log::info;
use naia_bevy_server::{Server, ServerAddrs};
use network_shared::{protocol::Protocol, Channels};
use std::collections::HashMap;
use crate::resources::Global;

pub fn init(mut commands: Commands, mut server: Server<Protocol, Channels>) {
    info!("Naia Server is running");

    let server_addresses = ServerAddrs::new(
        "127.0.0.1:14191"
            .parse()
            .expect("could not parse Signalling address/port"),
        "127.0.0.1:14192"
            .parse()
            .expect("could not parse WebRTC data address/port"),
        "127.0.0.1:14192",
    );

    server.listen(&server_addresses);

    let main_room_key = server.make_room().key();

    commands.insert_resource(Global {
        main_room_key,
        user_to_prediction_map: HashMap::new(),
    });
}
