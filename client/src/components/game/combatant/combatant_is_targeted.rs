use crate::store::game_store::get_current_party_option;
use crate::store::game_store::GameStore;
use common::combat::combat_actions::filter_possible_target_ids_by_prohibited_combatant_states::filter_possible_target_ids_by_prohibited_combatant_states;
use common::combat::combat_actions::CombatAction;
use common::combat::combat_actions::CombatActionTarget;
use common::combat::combat_actions::FriendOrFoe;
use std::rc::Rc;

pub fn combatant_targeted_by(
    game_state: Rc<GameStore>,
    combatant_id: &u32,
) -> Vec<(u32, CombatAction)> {
    // check if any entity in own party targeting
    // return vec of entities targeting and the ability name
    let mut to_return: Vec<(u32, CombatAction)> = Vec::new();
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
        if let Some(combat_action) = &character.combatant_properties.selected_combat_action {
            // GET THE COMBAT ACTION PROPERTIES SO WE CAN FILTER PROHIBITED TARGET STATES
            let selected_combat_action_properties_option =
                combat_action.get_properties_if_owned(game, *id).ok();

            if let Some(selected_combat_action_properties) =
                selected_combat_action_properties_option
            {
                let (filtered_character_ids, filtered_monster_ids_option) =
                    filter_possible_target_ids_by_prohibited_combatant_states(
                        game,
                        &selected_combat_action_properties.prohibited_target_combatant_states,
                        character_positions.clone(),
                        Some(monster_positions.clone()),
                    )
                    .ok()
                    .unwrap_or_else(|| (vec![], Some(vec![])));

                if let Some(targets) = &character.combatant_properties.combat_action_targets {
                    let is_targeted_by_this_character = match targets {
                        CombatActionTarget::Single(targeted_id) => combatant_id == targeted_id,
                        CombatActionTarget::Group(category) => match category {
                            FriendOrFoe::Friendly => filtered_character_ids.contains(combatant_id),
                            FriendOrFoe::Hostile => {
                                if let Some(monster_ids) = filtered_monster_ids_option {
                                    monster_ids.contains(combatant_id)
                                } else {
                                    false
                                }
                            }
                        },
                        CombatActionTarget::All => true,
                    };

                    if is_targeted_by_this_character {
                        to_return.push((*id, combat_action.clone()))
                    }
                }
            }
        }
    }
    to_return
}
