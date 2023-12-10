use crate::websocket_server::game_server::{
    getters::{get_mut_party_game_name_and_character_ids_from_actor_id, ActorIdAssociatedGameData},
    GameServer,
};
use common::{
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

        let new_targets_option = if packet.ability_name_option.is_none() {
            let character = party
                .get_mut_character_if_owned(player_character_ids_option, packet.character_id)?;
            character.combatant_properties.selected_ability_name = None;
            character.combatant_properties.ability_targets = None;
            None
        } else {
            let character = party
                .get_character_if_owned(player_character_ids_option.clone(), packet.character_id)?;
            let ability_name = packet.ability_name_option.clone().expect("is_none checked");
            let target_preferences = &character.combatant_properties.ability_target_preferences;
            let new_targets = ability_name.targets_by_saved_preference_or_default(
                character.entity_properties.id,
                &target_preferences,
                party,
            )?;
            let new_target_preferences =
                target_preferences.get_updated_preferences(&ability_name, &new_targets, party);
            let character = party
                .get_mut_character_if_owned(player_character_ids_option, packet.character_id)?;
            character.combatant_properties.selected_ability_name = Some(ability_name);
            character.combatant_properties.ability_target_preferences = new_target_preferences;
            Some(new_targets)
        };

        self.emit_packet(
            &current_game_name,
            &GameServerUpdatePackets::CharacterSelectedAbility(CharacterSelectedAbilityPacket {
                character_id: packet.character_id,
                ability_name_option: packet.ability_name_option,
                targets_option: new_targets_option,
            }),
            Some(actor_id),
        )
    }
}
