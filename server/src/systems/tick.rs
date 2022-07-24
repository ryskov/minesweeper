use bevy_ecs::system::{Query, ResMut};
use naia_bevy_server::Server;
use network_shared::{protocol::Protocol, Channels};

use crate::resources::Global;

pub fn tick(
    mut global: ResMut<Global>,
    mut server: Server<Protocol, Channels>
) {

    for (_, user_key, entity) in server.scope_checks() {

    }

    server.send_all_updates();
}
