use crate::channels::{Channels, CHANNEL_CONFIG};
use naia_shared::{LinkConditionerConfig, SharedConfig, SocketConfig};
use std::time::Duration;

pub fn shared_config() -> SharedConfig<Channels> {
    let tick_interval = Some(Duration::from_millis(20));

    let link_condition = Some(LinkConditionerConfig::average_condition());

    SharedConfig::new(
        SocketConfig::new(link_condition, None),
        CHANNEL_CONFIG,
        tick_interval,
        None,
    )
}
