use crate::comm_channels::messages_from_yew::MessageFromYew;
use crate::yew_app::components::bevy_messages_manager::send_message_to_bevy::send_message_to_bevy;
use crate::yew_app::store::bevy_communication_store::BevyCommunicationStore;
use crate::yew_app::store::game_store::DetailableEntities;
use crate::yew_app::store::game_store::GameStore;
use common::adventuring_party::AdventuringParty;
use common::app_consts::error_messages;
use common::character::Character;
use common::errors::AppError;
use common::game::getters::get_mut_party;
use common::game::getters::get_party;
use common::game::RoguelikeRacerGame;
use common::packets::server_to_client::CharacterEquippedItemPacket;
use common::packets::CharacterAndSlot;
use yewdux::Dispatch;

pub fn handle_character_equipped_item(
    game_dispatch: Dispatch<GameStore>,
    bevy_communication_dispatch: Dispatch<BevyCommunicationStore>,
    packet: CharacterEquippedItemPacket,
    player_username: &String,
) -> Result<(), AppError> {
    let CharacterEquippedItemPacket {
        character_id,
        item_id,
        alt_slot,
    } = packet;
    game_dispatch.reduce_mut(|game_store| -> Result<(), AppError> {
        let player_owns_character = game_store
            .get_current_party_mut()?
            .player_owns_character(player_username, character_id);
        let character = game_store.get_mut_character(character_id)?;

        let unequipped_item_ids = character
            .combatant_properties
            .equip_item(item_id, alt_slot)?;
        let item_to_select = match unequipped_item_ids.get(0) {
            Some(id) => {
                let mut item = None;
                for item_in_inventory in &character.combatant_properties.inventory.items {
                    if item_in_inventory.entity_properties.id == *id {
                        item = Some(item_in_inventory.clone())
                    }
                }
                item
            }
            None => None,
        };

        if player_owns_character {
            match item_to_select {
                Some(item) => {
                    game_store.selected_item = Some(item.clone());
                    game_store.detailed_entity = Some(DetailableEntities::Item(item.clone()));
                    game_store.hovered_entity = None;
                }
                None => (),
            }
        }

        Ok(())
    });

    bevy_communication_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        send_message_to_bevy(
            &store.transmitter_option,
            MessageFromYew::CombatantEquippedItem(packet.character_id, packet.item_id, alt_slot),
        )
    })
}

pub fn handle_character_unequipped_slot(
    game_dispatch: Dispatch<GameStore>,
    bevy_communication_dispatch: Dispatch<BevyCommunicationStore>,
    packet: CharacterAndSlot,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        let CharacterAndSlot { character_id, slot } = &packet;
        let character = store.get_mut_character(*character_id)?;
        character
            .combatant_properties
            .unequip_slots(&vec![slot.clone()], false);
        Ok(())
    });
    bevy_communication_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        send_message_to_bevy(
            &store.transmitter_option,
            MessageFromYew::CombatantUnequippedItem(packet.character_id, packet.slot),
        )
    })
}

impl GameStore {
    pub fn get_mut_character<'a>(
        &'a mut self,
        character_id: u32,
    ) -> Result<&'a mut Character, AppError> {
        let game = self.game.as_mut().ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::GAME_NOT_FOUND.to_string(),
        })?;
        let party_id = self.current_party_id.ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::MISSING_PARTY_REFERENCE.to_string(),
        })?;
        let party = get_mut_party(game, party_id)?;
        party
            .characters
            .get_mut(&character_id)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::CHARACTER_NOT_FOUND.to_string(),
            })
    }

    pub fn get_current_game<'a>(&'a self) -> Result<&'a RoguelikeRacerGame, AppError> {
        self.game.as_ref().ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::GAME_NOT_FOUND.to_string(),
        })
    }

    pub fn get_current_game_mut<'a>(&'a mut self) -> Result<&'a mut RoguelikeRacerGame, AppError> {
        self.game.as_mut().ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::GAME_NOT_FOUND.to_string(),
        })
    }

    pub fn get_current_party_mut<'a>(&'a mut self) -> Result<&'a mut AdventuringParty, AppError> {
        let current_party_id = self.current_party_id.ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::MISSING_PARTY_REFERENCE.to_string(),
        })?;
        let game = self.get_current_game_mut()?;
        get_mut_party(game, current_party_id)
    }

    pub fn get_current_party<'a>(&'a self) -> Result<&'a AdventuringParty, AppError> {
        let current_party_id = self.current_party_id.ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::MISSING_PARTY_REFERENCE.to_string(),
        })?;
        let game = self.get_current_game()?;
        get_party(game, current_party_id)
    }
}
