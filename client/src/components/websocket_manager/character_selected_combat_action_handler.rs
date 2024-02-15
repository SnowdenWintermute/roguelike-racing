use crate::store::game_store::GameStore;
use common::errors::AppError;
use common::game::getters::get_character;
use common::game::getters::get_mut_party;
use common::packets::client_to_server::CharacterAndCombatAction;
use std::collections::HashSet;
use yewdux::Dispatch;

pub fn character_selected_combat_action_handler(
    game_dispatch: Dispatch<GameStore>,
    packet: CharacterAndCombatAction,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|game_store| -> Result<(), AppError> {
        let CharacterAndCombatAction {
            character_id,
            combat_action_option,
        } = packet.clone();
        let party = game_store.get_current_party()?;
        let party_id = party.id;
        let game = game_store.get_current_game()?;
        let character = get_character(game, party_id, character_id)?;
        let username = character.name_of_controlling_user.clone();
        let battle_id_option = party.battle_id;
        let character_positions = party.character_positions.clone();
        let player_character_ids_option = &Some(HashSet::from([character_id])); // trust the server is sending valid packets

        let combat_action_properties_option = match combat_action_option {
            Some(combat_action) => {
                Some(combat_action.get_properties_if_owned(game, packet.character_id)?)
            }
            None => None,
        };

        let game = game_store.get_current_game_mut()?;
        game.assign_character_action_targets(
            packet.character_id,
            &player_character_ids_option, // trust the server is sending valid packets
            &username,
            party_id,
            battle_id_option,
            &character_positions,
            &combat_action_properties_option,
        )?;

        let party = get_mut_party(game, party_id)?;
        let character = party
            .get_mut_character_if_owned(player_character_ids_option.clone(), packet.character_id)?;
        character.combatant_properties.selected_combat_action = packet.combat_action_option.clone();

        Ok(())
    })
}
