use crate::store::game_store::get_current_party_option;
use crate::store::game_store::GameStore;
use common::combat::combat_actions::CombatActionTarget;
use common::combat::combat_actions::FriendOrFoe;
use common::combatants::abilities::CombatantAbilityNames;
use common::items::consumables::ConsumableProperties;
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

    for (id, character) in party.characters.iter() {
        if let Some(targets) = &character.combatant_properties.combat_action_targets {
            let is_targeted_by_this_character = match targets {
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
