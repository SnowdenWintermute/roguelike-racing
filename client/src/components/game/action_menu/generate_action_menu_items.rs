use super::available_actions::{GameActions, MenuTypes};
use crate::store::game_store::GameStore;
use common::adventuring_party::AdventuringParty;
use common::combatants::abilities::AbilityUsableContext;
use common::combatants::abilities::CombatantAbilityNames;
use std::rc::Rc;

pub fn generate_action_menu_items(
    game_state: &Rc<GameStore>,
    party: &AdventuringParty,
) -> Vec<GameActions> {
    let mut menu_items: Vec<MenuTypes> = Vec::new();
    let mut new_actions: Vec<GameActions> = Vec::new();

    if game_state.viewing_items_on_ground {
        menu_items.push(MenuTypes::ItemsOnGround);
        new_actions = MenuTypes::get_menu(&menu_items, None, None);
        //
    } else if game_state.selected_item.is_some() {
        let id = game_state
            .selected_item
            .clone()
            .expect("is_some checked")
            .entity_properties
            .id;
        menu_items.push(MenuTypes::ItemSelected(id));
        new_actions = MenuTypes::get_menu(&menu_items, None, None);
        //
    } else if game_state.viewing_equiped_items {
        menu_items.push(MenuTypes::ViewingEquipedItems);
        let focused_character = party.characters.get(&game_state.focused_character_id);
        if let Some(character) = focused_character {
            let mut ids = Vec::new();
            for (_slot, item) in &character.combatant_properties.equipment {
                ids.push(item.entity_properties.id);
            }
            new_actions = MenuTypes::get_menu(&menu_items, Some(ids), None)
        }
    } else if game_state.viewing_inventory {
        menu_items.push(MenuTypes::InventoryOpen);
        let focused_character = party.characters.get(&game_state.focused_character_id);
        if let Some(character) = focused_character {
            let mut ids = Vec::new();
            for item in &character.inventory.items {
                ids.push(item.entity_properties.id);
            }
            new_actions = MenuTypes::get_menu(&menu_items, Some(ids), None);
        }
        //
    } else if game_state.viewing_skill_level_up_menu
        || game_state.viewing_attribute_point_assignment_menu
    {
        menu_items.push(MenuTypes::LevelUpAbilities);
        let ability_names = get_ability_menu_names(&party, game_state.focused_character_id, None);
        new_actions = MenuTypes::get_menu(&menu_items, None, Some(ability_names));
        //
    } else if party.current_room.monsters.is_none() {
        menu_items.push(MenuTypes::OutOfCombat);
        let ability_names = get_ability_menu_names(
            &party,
            game_state.focused_character_id,
            Some(AbilityUsableContext::InCombat),
        );
        if party.current_room.treasure_chest.is_some() {
            menu_items.push(MenuTypes::UnopenedChest);
        }
        if party.current_room.items.is_some() {
            menu_items.push(MenuTypes::ItemsOnGround);
        }
        new_actions = MenuTypes::get_menu(&menu_items, None, Some(ability_names));
        //
    } else {
        menu_items.push(MenuTypes::InCombat);
        let ability_names = get_ability_menu_names(
            &party,
            game_state.focused_character_id,
            Some(AbilityUsableContext::OutOfCombat),
        );
        new_actions = MenuTypes::get_menu(&menu_items, None, Some(ability_names));
    }

    new_actions
}

fn get_ability_menu_names(
    party: &AdventuringParty,
    focused_character_id: u32,
    excluded_usable_context: Option<AbilityUsableContext>,
) -> Vec<CombatantAbilityNames> {
    let focused_character = party.characters.get(&focused_character_id);
    let mut ability_names = Vec::new();
    if let Some(character) = focused_character {
        for (ability_name, ability) in &character.combatant_properties.abilities {
            if let Some(excluded_context) = &excluded_usable_context {
                if &ability.usable_context != excluded_context {
                    ability_names.push(ability_name.clone());
                }
            }
        }
    }
    ability_names
}
