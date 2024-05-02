use bevy_app::bevy_main;
use comm_channels::comm_channel_bevy_plugin::CommChannelPlugin;
use comm_channels::create_comm_channels;
use yew_app::yew_main;
mod bevy_app;
mod comm_channels;
mod frontend_common;
pub mod utils;
mod yew_app;

fn main() {
    let (yew_channel, bevy_channel) = create_comm_channels();
    let comm_channel_bevy_plugin =
        CommChannelPlugin::new(bevy_channel.0.clone(), yew_channel.0.clone());

    yew_main(yew_channel.0, bevy_channel.0);
    bevy_main(comm_channel_bevy_plugin);
}
