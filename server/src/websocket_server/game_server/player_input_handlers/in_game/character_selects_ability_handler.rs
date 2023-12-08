use crate::websocket_server::game_server::{
    getters::{get_mut_party_game_name_and_character_ids_from_actor_id, ActorIdAssociatedGameData},
    GameServer,
};
use common::{
    app_consts::error_messages,
    combatants::abilities::get_combatant_ability_attributes::TargetingScheme,
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

        let new_targets_and_preferences_option = {
            if let Some(ability_name) = packet.ability_name_option {
                let target_preferences = &character.combatant_properties.ability_target_preferences;

                let new_targets = ability_name.targets_by_saved_preference_or_default(
                    character.entity_properties.id,
                    &target_preferences,
                    party,
                )?;

                let new_target_preferences =
                    target_preferences.get_updated_preferences(&ability_name, &targets, party);
                Some((new_targets, new_target_preferences))
            } else {
                None
            }
        };

        let character =
            party.get_mut_character_if_owned(player_character_ids_option, packet.character_id)?;
        character.combatant_properties.selected_ability_name = packet.ability_name_option.clone();

        if let Some((new_targets, new_target_preferences)) = new_targets_and_preferences_option {
            character.combatant_properties.ability_targets = Some(new_targets);
            character.combatant_properties.ability_target_preferences =
                Some(new_target_preferences);
        } else {
            character.combatant_properties.ability_targets = None
        }

        let targets_option = match new_targets_and_preferences_option {
            Some((new_targets, new_target_preferences)) => Some(new_targets),
            None => None,
        };

        self.emit_packet(
            &current_game_name,
            &GameServerUpdatePackets::CharacterSelectedAbility(CharacterSelectedAbilityPacket {
                character_id: packet.character_id,
                ability_name_option: packet.ability_name_option,
                targets_option,
            }),
            Some(actor_id),
        )
    }
}
