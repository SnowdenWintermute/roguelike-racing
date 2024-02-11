use crate::websocket_server::game_server::getters::get_mut_game_data_from_actor_id;
use crate::websocket_server::game_server::getters::ActorIdAssociatedGameData;
use crate::websocket_server::game_server::GameServer;
use common::errors::AppError;
use common::game::getters::get_mut_party;
use common::packets::client_to_server::CharacterAndCombatAction;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;

impl GameServer {
    pub fn character_selects_combat_action_handler(
        &mut self,
        actor_id: u32,
        packet: CharacterAndCombatAction,
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

        let combat_action_properties_option = match &packet.combat_action_option {
            Some(combat_action) => {
                Some(combat_action.get_properties_if_owned(game, packet.character_id)?)
            }
            None => None,
        };

        let _ = game.assign_character_initial_targets_on_combat_action_selection(
            packet.character_id,
            &player_character_ids_option,
            party_id,
            battle_id_option,
            &character_positions,
            &combat_action_properties_option,
        )?;

        let party = get_mut_party(game, party_id)?;
        let character = party
            .get_mut_character_if_owned(player_character_ids_option.clone(), packet.character_id)?;
        character.combatant_properties.selected_combat_action = packet.combat_action_option.clone();

        self.emit_packet(
            &party_websocket_channel_name,
            &WebsocketChannelNamespace::Party,
            &GameServerUpdatePackets::CharacterSelectedCombatAction(CharacterAndCombatAction {
                character_id: packet.character_id,
                combat_action_option: packet.combat_action_option,
            }),
            None,
        )
    }
}
