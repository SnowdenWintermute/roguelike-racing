use crate::{
    components::websocket_manager::send_client_input::send_client_input,
    store::game_store::GameStore,
};
use common::{
    combatants::abilities::CombatantAbilityNames,
    game::getters::get_mut_party,
    packets::client_to_server::{ClientSelectAbilityPacket, PlayerInputs},
};
use web_sys::WebSocket;
use yewdux::prelude::Dispatch;

pub fn handle_select_ability(
    game_dispatch: Dispatch<GameStore>,
    websocket_option: &Option<WebSocket>,
    ability_name: CombatantAbilityNames,
) {
    game_dispatch.reduce_mut(|game_store| {
        let game = game_store
            .game
            .as_mut()
            .expect("only use abilities in game");
        let party_id = game_store
            .current_party_id
            .expect("only use abilities in party");
        let party = get_mut_party(game, party_id).expect("only use in party");
        let focused_character = party
            .characters
            .get(&game_store.focused_character_id)
            .expect("to have a character");
        let ability = focused_character
            .combatant_properties
            .abilities
            .get(&ability_name)
            .expect("the character to have selected an ability they own");
        let previous_targets_are_still_valid =
            ability.targets_are_valid(&ability.most_recently_targeted, &party);

        let new_target_ids = if previous_targets_are_still_valid {
            ability.most_recently_targeted.clone()
        } else {
            ability
                .get_default_target_ids(&party, game_store.focused_character_id)
                .clone()
        };

        let focused_character = party
            .characters
            .get_mut(&game_store.focused_character_id)
            .expect("to have a character");
        focused_character.combatant_properties.selected_ability_name = Some(ability_name.clone());
        focused_character.combatant_properties.ability_target_ids = new_target_ids.clone();
        let ability = focused_character
            .combatant_properties
            .abilities
            .get_mut(&ability_name)
            .expect("the character to have selected an ability they own");
        ability.most_recently_targeted = new_target_ids.clone();

        send_client_input(
            websocket_option,
            PlayerInputs::SelectAbility(ClientSelectAbilityPacket {
                character_id: focused_character.entity_properties.id,
                ability_name_option: Some(ability_name),
            }),
        );
    })
}
