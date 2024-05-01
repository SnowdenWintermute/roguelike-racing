use super::enums::GameActions;
use crate::yew_app::store::game_store::get_current_battle_option;
use crate::yew_app::store::game_store::GameStore;
use crate::yew_app::store::lobby_store::LobbyStore;
use common::combat::combat_actions::CombatAction;
use common::game::getters::get_character;
use std::ops::Deref;
use std::rc::Rc;

pub fn determine_action_menu_buttons_disabled(
    action: &GameActions,
    game_state: &Rc<GameStore>,
    lobby_state: &Rc<LobbyStore>,
) -> bool {
    let game_option = &game_state.deref().game.as_ref();
    if game_option.is_none() {
        return true;
    }
    let game = game_option.expect("none checked");
    let battle_option = get_current_battle_option(&game_state);
    let current_party_id = game_state.clone().current_party_id.expect("party to exist");
    let party = game
        .adventuring_parties
        .get(&current_party_id)
        .expect("to have valid party ref");
    let focused_character_id = game_state.clone().focused_character_id;
    let focused_character_result = get_character(game, current_party_id, focused_character_id);
    if focused_character_result.is_err() {
        return true;
    }
    let focused_character = focused_character_result.expect("is_none checked");

    let player_owns_character =
        party.player_owns_character(&lobby_state.username, focused_character_id);

    match action {
        GameActions::UseItem(_) => {
            if !player_owns_character {
                return true;
            }
            let item = &game_state
                .deref()
                .selected_item
                .as_ref()
                .expect("button should only be shown when item is selected");
            if !focused_character.combatant_properties.can_use_item(&item)
                && focused_character
                    .combatant_properties
                    .slot_item_is_equipped(&item.entity_properties.id)
                    .is_none()
            {
                return true;
            }
            false
        }
        GameActions::TakeItem => true,
        GameActions::DropItem(_) => {
            if !player_owns_character {
                return true;
            }
            false
        }
        GameActions::SelectCombatAction(_) => {
            if !player_owns_character {
                return true;
            }
            if let Some(battle) = battle_option {
                if !battle.combatant_is_first_in_turn_order(focused_character_id) {
                    return true;
                }
            }
            if game_state
                .combatants_animating
                .contains(&focused_character_id)
            {
                return true;
            }
            false
        }
        GameActions::UseSelectedCombatAction => {
            focused_character.combatant_properties.hit_points == 0 || {
                if let Some(action) = &focused_character
                    .combatant_properties
                    .selected_combat_action
                {
                    let mp_cost = match action {
                        CombatAction::AbilityUsed(ability_name) => focused_character
                            .combatant_properties
                            .get_ability_cost_if_owned(&ability_name)
                            .expect("to own the ability"),
                        CombatAction::ConsumableUsed(_) => 0,
                    };
                    mp_cost as u16 > focused_character.combatant_properties.mana
                } else {
                    true
                }
            }
        }
        GameActions::ShardItem(_) => !player_owns_character || true,
        GameActions::AssignAttributePoint(_) => {
            focused_character
                .combatant_properties
                .unspent_attribute_points
                <= 0
        }
        GameActions::SetAssignAttributePointsMenuOpen(_) => !player_owns_character,

        _ => false,
    }
}
