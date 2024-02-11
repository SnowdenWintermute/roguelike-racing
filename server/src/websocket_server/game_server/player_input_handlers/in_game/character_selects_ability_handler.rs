use crate::websocket_server::game_server::getters::get_mut_game_data_from_actor_id;
use crate::websocket_server::game_server::getters::ActorIdAssociatedGameData;
use crate::websocket_server::game_server::GameServer;
use common::errors::AppError;
use common::game::getters::get_mut_party;
use common::packets::client_to_server::ClientSelectAbilityPacket;
use common::packets::server_to_client::CharacterSelectedAbilityPacket;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;

impl GameServer {
    pub fn character_selects_ability_handler(
        &mut self,
        actor_id: u32,
        packet: ClientSelectAbilityPacket,
    ) -> Result<(), AppError> {
        let ActorIdAssociatedGameData {
            game,
            party_id,
            player_character_ids_option,
            ..
        } = get_mut_game_data_from_actor_id(self, actor_id)?;
        let party = get_mut_party(game, party_id)?;
        let battle_id_option = party.battle_id;
        let party_websocket_channel_name = party.websocket_channel_name.clone();
        let character_positions = party.character_positions.clone();
        let character = party
            .get_mut_character_if_owned(player_character_ids_option.clone(), packet.character_id)?;

        let combat_action_properties_option =
            if let Some(ability_name) = packet.ability_name_option.clone() {
                // don't allow selection of unowned ability
                let _ = character
                    .combatant_properties
                    .get_mut_ability_if_owned(&ability_name)?;
                let ability_attributes = ability_name.get_attributes();
                Some(ability_attributes.combat_action_properties)
            } else {
                None
            };

        let new_targets_option = game.assign_character_initial_targets_on_combat_action_selection(
            packet.character_id,
            &player_character_ids_option,
            party_id,
            battle_id_option,
            &character_positions,
            &combat_action_properties_option,
        )?;
        println!(
            "assigned targets on combat action selection: {:#?}",
            new_targets_option
        );

        let party = get_mut_party(game, party_id)?;
        let character = party
            .get_mut_character_if_owned(player_character_ids_option.clone(), packet.character_id)?;
        character.combatant_properties.selected_ability_name = packet.ability_name_option.clone();

        self.emit_packet(
            &party_websocket_channel_name,
            &WebsocketChannelNamespace::Party,
            &GameServerUpdatePackets::CharacterSelectedAbility(CharacterSelectedAbilityPacket {
                character_id: packet.character_id,
                ability_name_option: packet.ability_name_option,
                targets_option: new_targets_option,
            }),
            None,
        )
    }
}
