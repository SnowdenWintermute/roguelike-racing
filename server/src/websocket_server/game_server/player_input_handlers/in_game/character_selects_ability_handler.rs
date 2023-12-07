use crate::websocket_server::game_server::{
    getters::{get_mut_party_game_name_and_character_ids_from_actor_id, ActorIdAssociatedGameData},
    GameServer,
};
use common::{
    app_consts::error_messages,
    errors::AppError,
    packets::{
        client_to_server::ClientSelectAbilityPacket,
        server_to_client::{CharacterSelectedAbilityPacket, GameServerUpdatePackets},
    },
};

impl GameServer {
    pub fn character_selects_ability_handler(
        &mut self,
        actor_id: u32,
        packet: ClientSelectAbilityPacket,
    ) -> Result<(), AppError> {
        let ActorIdAssociatedGameData {
            party,
            player_character_ids_option,
            current_game_name,
            ..
        } = get_mut_party_game_name_and_character_ids_from_actor_id(self, actor_id)?;

        let character = party
            .get_character_if_owned(player_character_ids_option.clone(), packet.character_id)?;

        let new_target_ids = match &packet.ability_name_option {
            Some(ability_name) => {
                // check if ability is valid
                let ability = character
                    .combatant_properties
                    .abilities
                    .get(&ability_name)
                    .ok_or_else(|| AppError {
                        error_type: common::errors::AppErrorTypes::InvalidInput,
                        message: error_messages::ABILITY_NOT_OWNED.to_string(),
                    })?;
                let cloned_ability = ability.clone();
                let previous_targets_are_still_valid = cloned_ability
                    .targets_are_valid(&cloned_ability.most_recently_targeted, &party);

                let new_target_ids = if previous_targets_are_still_valid {
                    cloned_ability.most_recently_targeted.clone()
                } else {
                    cloned_ability
                        .get_default_target_ids(&party, packet.character_id)
                        .clone()
                };
                new_target_ids
            }
            None => None,
        };

        let character =
            party.get_mut_character_if_owned(player_character_ids_option, packet.character_id)?;

        let character_id = character.entity_properties.id;

        // set the new targets
        character.combatant_properties.ability_target_ids = new_target_ids.clone();
        // set the new ability selected
        character.combatant_properties.selected_ability_name = packet.ability_name_option.clone();
        // save prev targets
        if let Some(ability_name) = &packet.ability_name_option {
            let ability = character
                .combatant_properties
                .abilities
                .get_mut(&ability_name)
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::InvalidInput,
                    message: error_messages::ABILITY_NOT_OWNED.to_string(),
                })?;

            ability.most_recently_targeted = new_target_ids.clone();
        }

        self.emit_packet(
            &current_game_name,
            &GameServerUpdatePackets::CharacterSelectedAbility(CharacterSelectedAbilityPacket {
                character_id,
                ability_name_option: packet.ability_name_option,
                target_ids_option: new_target_ids,
            }),
            Some(actor_id),
        )
    }
}
