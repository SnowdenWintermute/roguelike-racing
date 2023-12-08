use crate::{
    components::websocket_manager::send_client_input::send_client_input,
    store::game_store::GameStore,
};
use common::{
    combatants::abilities::{
        get_combatant_ability_attributes::{TargetCategories, TargetingScheme},
        targets_are_valid::is_id_of_existing_opponent,
    },
    game::getters::get_mut_party,
    packets::client_to_server::{ClientChangeTargetsPacket, PlayerInputs},
    primatives::NextOrPrevious,
};
use web_sys::WebSocket;
use yewdux::prelude::Dispatch;

pub fn handle_cycle_targets(
    game_dispatch: Dispatch<GameStore>,
    websocket_option: &Option<WebSocket>,
    direction: &NextOrPrevious,
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
        let ability_name = focused_character
            .combatant_properties
            .selected_ability_name
            .as_ref()
            .expect("to have an ability selected");
        let ability = focused_character
            .combatant_properties
            .abilities
            .get(ability_name)
            .expect("the character to have selected an ability they own");

        let ability_attributes = ability.ability_name.get_attributes();
        let most_recently_targeted = match ability.selected_targeting_scheme {
            TargetingScheme::Single => ability.most_recently_targeted_single,
            TargetingScheme::Area => ability.most_recently_targeted_area,
        };
        let current_target_is_valid = ability.targets_are_valid(&most_recently_targeted, &party);

        let current_target_ids = if !current_target_is_valid {
            ability
                .get_default_target_ids(&party, focused_character.entity_properties.id)
                .expect("to get valid default target ids")
        } else {
            most_recently_targeted.clone().expect("to have valid ids")
        };

        let new_target_ids = match ability.selected_targeting_scheme {
            TargetingScheme::Single => match ability_attributes.valid_target_categories {
                TargetCategories::Opponent => {
                    let monster_ids = party
                        .current_room
                        .monsters
                        .as_ref()
                        .expect("to be in combat")
                        .iter()
                        .map(|monster| monster.entity_properties.id)
                        .collect::<Vec<u32>>();

                    vec![get_next_or_prev_id_from_ordered_id_list(
                        &monster_ids,
                        &current_target_ids,
                        &direction,
                    )]
                }
                TargetCategories::User => current_target_ids,
                TargetCategories::Friendly => {
                    vec![get_next_or_prev_id_from_ordered_id_list(
                        &party.character_positions,
                        &current_target_ids,
                        direction,
                    )]
                }
                TargetCategories::Any => {
                    let monster_ids = party
                        .current_room
                        .monsters
                        .as_ref()
                        .expect("to be in combat")
                        .iter()
                        .map(|monster| monster.entity_properties.id)
                        .collect::<Vec<u32>>();
                    let mut all_combatant_ids = monster_ids.clone();
                    let mut cloned_character_positions = party.character_positions.clone();
                    all_combatant_ids.append(&mut cloned_character_positions);
                    vec![get_next_or_prev_id_from_ordered_id_list(
                        &all_combatant_ids,
                        &current_target_ids,
                        direction,
                    )]
                }
            },
            TargetingScheme::Area => match ability_attributes.valid_target_categories {
                TargetCategories::Opponent
                | TargetCategories::User
                | TargetCategories::Friendly => current_target_ids,
                TargetCategories::Any => {
                    if is_id_of_existing_opponent(&party, &current_target_ids[0]) {
                        party.character_positions.clone()
                    } else {
                        party
                            .current_room
                            .monsters
                            .as_ref()
                            .expect("to be in combat")
                            .iter()
                            .map(|monster| monster.entity_properties.id)
                            .collect::<Vec<u32>>()
                    }
                }
            },
        };

        send_client_input(
            &websocket_option,
            PlayerInputs::ChangeTargetIds(ClientChangeTargetsPacket {
                character_id: focused_character.entity_properties.id,
                target_ids: new_target_ids,
            }),
        )
    });
}

fn get_next_or_prev_id_from_ordered_id_list(
    possible_target_ids: &Vec<u32>,
    current_target_ids: &Vec<u32>,
    direction: &NextOrPrevious,
) -> u32 {
    let current_position_index = {
        let mut to_return = 0;
        for (index, id) in possible_target_ids.iter().enumerate() {
            if id == &current_target_ids[0] {
                to_return = index;
                break;
            }
        }
        to_return
    };

    let new_index = match direction {
        NextOrPrevious::Next => {
            if current_position_index < possible_target_ids.len() - 1 {
                current_position_index + 1
            } else {
                0
            }
        }
        NextOrPrevious::Previous => {
            if current_position_index > 0 {
                current_position_index - 1
            } else {
                possible_target_ids.len() - 1
            }
        }
    };

    possible_target_ids[new_index]
}
