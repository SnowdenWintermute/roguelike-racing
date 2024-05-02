use crate::comm_channels::messages_from_yew::MessageFromYew;
use crate::yew_app::components::bevy_messages_manager::send_message_to_bevy::send_message_to_bevy;
use crate::yew_app::store::bevy_communication_store::BevyCommunicationStore;
use crate::yew_app::store::game_store::GameStore;
use common::errors::AppError;
use common::packets::CharacterAndItem;
use yewdux::Dispatch;

pub fn handle_character_picked_up_item(
    game_dispatch: Dispatch<GameStore>,
    bevy_communication_dispatch: Dispatch<BevyCommunicationStore>,
    packet: CharacterAndItem,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        let party = store.get_current_party_mut()?;
        let item_picked_up = party.remove_item_from_ground(packet.item_id)?;
        bevy_communication_dispatch.reduce_mut(|store| -> Result<(), AppError> {
            send_message_to_bevy(
                &store.transmitter_option,
                MessageFromYew::CombatantPickedUpItem(packet.character_id, item_picked_up.clone()),
            )
        })?;
        let character = store.get_mut_character(packet.character_id)?;
        character
            .combatant_properties
            .inventory
            .items
            .push(item_picked_up);

        Ok(())
    })
}
