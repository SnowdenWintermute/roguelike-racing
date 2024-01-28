use crate::store::game_store::get_active_combatant;
use crate::store::game_store::get_current_party_option;
use crate::store::game_store::GameStore;
use common::combat::combat_actions::CombatActionTarget;
use common::combat::combat_actions::FriendOrFoe;
use std::rc::Rc;

pub fn combatant_is_targeted(game_state: Rc<GameStore>, combatant_id: &u32) -> bool {
    let party_option = get_current_party_option(&game_state);
    if party_option.is_none() {
        return false;
    }
    let party = party_option.expect("is_none checked");
    let active_character_result = get_active_combatant(&game_state);
    if let Ok(active_character_option) = active_character_result {
        if let Some(active_character) = active_character_option {
            let targets_option = &active_character.1.ability_targets;
            if let Some(targets) = targets_option {
                match targets {
                    CombatActionTarget::Single(targeted_id) => combatant_id == targeted_id,
                    CombatActionTarget::Group(category) => match category {
                        FriendOrFoe::Friendly => party.character_positions.contains(combatant_id),
                        FriendOrFoe::Hostile => {
                            if let Ok(monster_ids) = party.get_monster_ids() {
                                monster_ids.contains(combatant_id)
                            } else {
                                false
                            }
                        }
                    },
                    CombatActionTarget::All => true,
                }
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}
