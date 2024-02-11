use crate::store::game_store::get_current_party_option;
use crate::store::game_store::GameStore;
use common::combat::combat_actions::filter_possible_target_ids_by_prohibited_combatant_states::filter_possible_target_ids_by_prohibited_combatant_states;
use common::combat::combat_actions::CombatActionTarget;
use common::combat::combat_actions::FriendOrFoe;
use common::combatants::abilities::CombatantAbilityNames;
use common::items::consumables::ConsumableProperties;
use gloo::console::log;
use std::rc::Rc;

pub fn combatant_targeted_by(
    game_state: Rc<GameStore>,
    combatant_id: &u32,
) -> Vec<(
    u32,
    Option<CombatantAbilityNames>,
    Option<ConsumableProperties>,
)> {
    // check if any entity in own party targeting
    // return vec of entities targeting and the ability name
    let mut to_return: Vec<(
        u32,
        Option<CombatantAbilityNames>,
        Option<ConsumableProperties>,
    )> = Vec::new();
    let party_option = get_current_party_option(&game_state);
    if party_option.is_none() {
        return Vec::new();
    }
    let party = party_option.expect("is_none checked");
    let character_positions = party.character_positions.clone();
    let monster_positions = party
        .get_monster_ids()
        .ok()
        .clone()
        .unwrap_or_else(|| vec![]);
    let game = game_state.get_current_game().expect("to be in a game");

    for (id, character) in party.characters.iter() {
        // GET THE COMBAT ACTION PROPERTIES SO WE CAN FILTER PROHIBITED TARGET STATES
        let selected_combat_action_properties_option = if let Some(consumable_id) =
            character.combatant_properties.selected_consumable
        {
            let consumable_option = character
                .combatant_properties
                .inventory
                .get_consumable(&consumable_id)
                .ok();
            if let Some(consumable_properties) = consumable_option {
                Some(
                    consumable_properties
                        .consumable_type
                        .get_combat_action_properties(),
                )
            } else {
                None
            }
        } else if let Some(ability_name) = &character.combatant_properties.selected_ability_name {
            Some(ability_name.get_attributes().combat_action_properties)
        } else {
            None
        };
        let (filtered_character_ids, filtered_monster_ids_option) =
            if let Some(selected_combat_action_properties) =
                selected_combat_action_properties_option
            {
                filter_possible_target_ids_by_prohibited_combatant_states(
                    game,
                    &selected_combat_action_properties.prohibited_target_combatant_states,
                    character_positions.clone(),
                    Some(monster_positions.clone()),
                )
                .ok()
                .unwrap_or_else(|| (vec![], Some(vec![])))
            } else {
                (vec![], Some(vec![]))
            };
        if let Some(targets) = &character.combatant_properties.combat_action_targets {
            let is_targeted_by_this_character = match targets {
                CombatActionTarget::Single(targeted_id) => combatant_id == targeted_id,
                CombatActionTarget::Group(category) => match category {
                    FriendOrFoe::Friendly => filtered_character_ids.contains(combatant_id),
                    FriendOrFoe::Hostile => {
                        if let Some(monster_ids) = filtered_monster_ids_option {
                            // log!(format!(
                            //     "monster ids: {:?}, monster {} is targeted",
                            //     monster_ids, combatant_id
                            // ));
                            monster_ids.contains(combatant_id)
                        } else {
                            false
                        }
                    }
                },
                CombatActionTarget::All => true,
            };
            let consumable_type_option =
                if let Some(consumable_id) = character.combatant_properties.selected_consumable {
                    character
                        .combatant_properties
                        .inventory
                        .get_consumable(&consumable_id)
                        .cloned()
                        .ok()
                } else {
                    None
                };
            if is_targeted_by_this_character {
                to_return.push((
                    *id,
                    character.combatant_properties.selected_ability_name.clone(),
                    consumable_type_option,
                ))
            }
        }
    }

    to_return
}
