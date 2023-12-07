use crate::websocket_server::game_server::{
    getters::{get_mut_party_game_name_and_character_ids_from_actor_id, ActorIdAssociatedGameData},
    GameServer,
};
use common::{
    errors::AppError,
    packets::{
        client_to_server::ClientChangeTargetsPacket, server_to_client::GameServerUpdatePackets,
    },
};

impl GameServer {
    pub fn character_changes_ability_targets_handler(
        &mut self,
        actor_id: u32,
        packet: ClientChangeTargetsPacket,
    ) -> Result<(), AppError> {
        let ActorIdAssociatedGameData {
            party,
            player_character_ids_option,
            current_game_name,
            ..
        } = get_mut_party_game_name_and_character_ids_from_actor_id(self, actor_id)?;
        let ClientChangeTargetsPacket {
            character_id,
            target_ids,
        } = packet;
        let ability =
            party.get_character_selected_ability(player_character_ids_option, character_id)?;
        let targets_are_valid = ability.targets_are_valid(&Some(target_ids.clone()), &party);

        let new_target_ids = if targets_are_valid {
            Some(target_ids)
        } else {
            ability.get_default_target_ids(party, character_id)
        };

        let ActorIdAssociatedGameData {
            party,
            player_character_ids_option,
            ..
        } = get_mut_party_game_name_and_character_ids_from_actor_id(self, actor_id)?;
        let ability = party.get_mut_character_selected_ability(
            player_character_ids_option.clone(),
            character_id,
        )?;
        ability.most_recently_targeted = new_target_ids.clone();
        let character =
            party.get_mut_character_if_owned(player_character_ids_option, character_id)?;
        character.combatant_properties.ability_target_ids = new_target_ids.clone();

        self.emit_packet(
            &current_game_name,
            &GameServerUpdatePackets::CharacterChangedTargets(ClientChangeTargetsPacket {
                character_id,
                target_ids: new_target_ids.unwrap_or(vec![]),
            }),
            None,
        )
    }
}
