use bevy_ecs::{event::EventReader, system::ResMut};
use bevy_log::info;

use naia_bevy_server::{
    events::{AuthorizationEvent, ConnectionEvent, DisconnectionEvent, MessageEvent},
    shared::Random,
    Server,
};

use network_shared::{protocol::Protocol, Channels};

use crate::resources::Global;

pub fn authorization_event(
    mut event_reader: EventReader<AuthorizationEvent<Protocol>>,
    mut server: Server<Protocol, Channels>,
) {
    for event in event_reader.iter() {
        if let AuthorizationEvent(user_key, Protocol::Auth(auth)) = event {
            if *auth.username == "charlie" && *auth.password == "12345" {
                bevy_log::info!("Accepting connection (auth)!");
                server.accept_connection(user_key);
            } else {
                server.reject_connection(user_key);
            }
        }
    }
}

pub fn connection_event<'world, 'state>(
    mut event_reader: EventReader<ConnectionEvent>,
    mut global: ResMut<Global>,
    mut server: Server<'world, 'state, Protocol, Channels>,
) {
    for event in event_reader.iter() {
        let ConnectionEvent(user_key) = event;
        let address = server 
            .user_mut(user_key)
            .enter_room(&global.main_room_key)
            .address();

        info!("Naia Server connected to: {}", address);

        
    }
}
