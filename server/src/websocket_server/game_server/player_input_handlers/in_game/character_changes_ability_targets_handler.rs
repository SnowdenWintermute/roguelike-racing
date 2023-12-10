use crate::websocket_server::game_server::{
    getters::{get_mut_party_game_name_and_character_ids_from_actor_id, ActorIdAssociatedGameData},
    GameServer,
};
use common::{
    errors::AppError,
    packets::{client_to_server::ChangeTargetsPacket, server_to_client::GameServerUpdatePackets},
};

impl GameServer {
    pub fn character_changes_ability_targets_handler(
        &mut self,
        actor_id: u32,
        packet: ChangeTargetsPacket,
    ) -> Result<(), AppError> {
        let ActorIdAssociatedGameData {
            party,
            player_character_ids_option,
            current_game_name,
            ..
        } = get_mut_party_game_name_and_character_ids_from_actor_id(self, actor_id)?;
        let ChangeTargetsPacket {
            character_id,
            new_targets,
        } = packet;
        let ability =
            party.get_character_selected_ability(player_character_ids_option, character_id)?;
        let ability_name = ability.ability_name.clone();

        let new_targets =
            if ability
                .ability_name
                .targets_are_valid(character_id, &new_targets, party)
            {
                new_targets
            } else {
                ability
                    .ability_name
                    .get_default_targets(character_id, party)?
            };

        let ActorIdAssociatedGameData {
            party,
            player_character_ids_option,
            ..
        } = get_mut_party_game_name_and_character_ids_from_actor_id(self, actor_id)?;
        let character =
            party.get_character_if_owned(player_character_ids_option.clone(), character_id)?;

        let target_preferences = &character.combatant_properties.ability_target_preferences;
        let new_target_preferences =
            target_preferences.get_updated_preferences(&ability_name, &new_targets, party);
        let character =
            party.get_mut_character_if_owned(player_character_ids_option, character_id)?;

        character.combatant_properties.ability_target_preferences = new_target_preferences;
        character.combatant_properties.ability_targets = Some(new_targets.clone());

        self.emit_packet(
            &current_game_name,
            &GameServerUpdatePackets::CharacterChangedTargets(ChangeTargetsPacket {
                character_id,
                new_targets,
            }),
            None,
        )
    }
}
