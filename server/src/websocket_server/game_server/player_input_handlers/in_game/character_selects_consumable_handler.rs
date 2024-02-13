use crate::websocket_server::game_server::getters::get_mut_game_data_from_actor_id;
use crate::websocket_server::game_server::getters::ActorIdAssociatedGameData;
use crate::websocket_server::game_server::GameServer;
use common::app_consts::error_messages;
use common::combat::combat_actions::filter_possible_target_ids_by_prohibited_combatant_states::filter_possible_target_ids_by_prohibited_combatant_states;
use common::errors::AppError;
use common::errors::AppErrorTypes;
use common::game::getters::get_mut_party;
use common::packets::server_to_client::CharacterSelectedConsumablePacket;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;

impl GameServer {
    pub fn character_selects_consumable_handler(
        &mut self,
        actor_id: u32,
        packet: ClientSelectConsumablePacket,
    ) -> Result<(), AppError> {
        let ActorIdAssociatedGameData {
            game,
            party_id,
            player_character_ids_option,
            ..
        } = get_mut_game_data_from_actor_id(self, actor_id)?;

        let party = get_mut_party(game, party_id)?;
        let party_websocket_channel_name = party.websocket_channel_name.clone();

        let new_targets_option = if let Some(item_id) = packet.consumable_id_option {
            let battle_id_option = party.battle_id;
            let character_positions = party.character_positions.clone();
            let character = party.get_mut_character_if_owned(
                player_character_ids_option.clone(),
                packet.character_id,
            )?;
            let target_preferences = character
                .combatant_properties
                .combat_action_target_preferences
                .clone();

            let consumable_properties = character
                .combatant_properties
                .inventory
                .get_consumable(&item_id)?;

            let combat_action_properties = consumable_properties
                .consumable_type
                .get_combat_action_properties();

            let (ally_ids, opponent_ids_option) = if let Some(battle_id) = battle_id_option {
                let battle = game.battles.get(&battle_id).ok_or_else(|| AppError {
                    error_type: AppErrorTypes::Generic,
                    message: error_messages::BATTLE_NOT_FOUND.to_string(),
                })?;

                battle.get_ally_ids_and_opponent_ids_option(packet.character_id)?
            } else {
                (character_positions, None)
            };

            let prohibited_target_combatant_states = combat_action_properties
                .prohibited_target_combatant_states
                .clone();

            let (ally_ids, opponent_ids_option) =
                filter_possible_target_ids_by_prohibited_combatant_states(
                    game,
                    &prohibited_target_combatant_states,
                    ally_ids,
                    opponent_ids_option,
                )?;

            let party = get_mut_party(game, party_id)?;
            let character = party.get_mut_character_if_owned(
                player_character_ids_option.clone(),
                packet.character_id,
            )?;

            let new_targets = combat_action_properties.targets_by_saved_preference_or_default(
                character.entity_properties.id,
                &target_preferences,
                &ally_ids,
                &opponent_ids_option,
            )?;

            let new_target_preferences = target_preferences.get_updated_preferences(
                &combat_action_properties,
                &new_targets,
                ally_ids,
                opponent_ids_option,
            );
            let party = get_mut_party(game, party_id)?;
            let character = party
                .get_mut_character_if_owned(player_character_ids_option, packet.character_id)?;
            character.combatant_properties.selected_consumable = Some(item_id);
            character
                .combatant_properties
                .combat_action_target_preferences = new_target_preferences;
            character.combatant_properties.combat_action_targets = Some(new_targets.clone());
            Some(new_targets)
        } else {
            let character = party
                .get_mut_character_if_owned(player_character_ids_option, packet.character_id)?;
            character.combatant_properties.selected_consumable = None;
            character.combatant_properties.combat_action_targets = None;
            None
        };

        self.emit_packet(
            &party_websocket_channel_name,
            &WebsocketChannelNamespace::Party,
            &GameServerUpdatePackets::CharacterSelectedConsumable(
                CharacterSelectedConsumablePacket {
                    character_id: packet.character_id,
                    consumable_id_option: packet.consumable_id_option,
                    targets_option: new_targets_option,
                },
            ),
            Some(actor_id),
        )
    }
}
