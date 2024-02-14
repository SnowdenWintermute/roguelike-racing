use super::enums::GameActions;
use super::enums::MenuTypes;
use crate::store::game_store::GameStore;
use crate::store::lobby_store::LobbyStore;
use common::adventuring_party::AdventuringParty;
use common::combat::combat_actions::AbilityUsableContext;
use common::combatants::abilities::CombatantAbilityNames;
use common::dungeon_rooms::DungeonRoomTypes;
use common::items::consumables::ConsumableTypes;
use common::items::ItemProperties;
use std::collections::HashMap;
use std::rc::Rc;

// determine menu actions
pub fn determine_menu_actions(
    game_state: &Rc<GameStore>,
    lobby_state: &Rc<LobbyStore>,
    party: &AdventuringParty,
) -> Vec<GameActions> {
    let mut menu_types: Vec<MenuTypes> = Vec::new();
    let mut new_actions: Vec<GameActions> = Vec::new();

    let inventory_is_open = game_state.viewing_inventory;
    let focused_character_option = party.characters.get(&game_state.focused_character_id);
    let player_owns_character =
        party.player_owns_character(&lobby_state.username, game_state.focused_character_id);
    let focused_character_selected_combat_action_properties_option = match focused_character_option
    {
        Some(character) => {
            let combat_action_option = character
                .combatant_properties
                .selected_combat_action
                .clone();
            match combat_action_option {
                Some(combat_action) => match combat_action.get_properties_if_owned(
                    &game_state.game.as_ref().expect("to be in game"),
                    character.entity_properties.id,
                ) {
                    Ok(properties) => Some(properties),
                    Err(_) => None,
                },
                None => None,
            }
        }
        None => None,
    };

    if focused_character_selected_combat_action_properties_option.is_some() && player_owns_character
    {
        if let Some(combat_action_properties) =
            focused_character_selected_combat_action_properties_option
        {
            menu_types.push(MenuTypes::CombatActionSelected);
            new_actions = MenuTypes::get_actions(
                &menu_types,
                None,
                None,
                Some(combat_action_properties),
                inventory_is_open,
            );
        }
    } else if game_state.viewing_items_on_ground {
        menu_types.push(MenuTypes::ItemsOnGround);
        new_actions = MenuTypes::get_actions(&menu_types, None, None, None, inventory_is_open);
    } else if let Some(selected_item) = &game_state.selected_item {
        let id = selected_item.clone().entity_properties.id;
        menu_types.push(MenuTypes::ItemSelected(id));
        new_actions = MenuTypes::get_actions(&menu_types, None, None, None, inventory_is_open);
    } else if game_state.viewing_equipped_items {
        menu_types.push(MenuTypes::ViewingEquipedItems);
        let focused_character = party.characters.get(&game_state.focused_character_id);
        if let Some(character) = focused_character {
            let mut ids = Vec::new();
            for (_slot, item) in &character.combatant_properties.equipment {
                ids.push(item.entity_properties.id);
            }
            new_actions = MenuTypes::get_actions(
                &menu_types,
                Some((HashMap::new(), ids)),
                None,
                None,
                inventory_is_open,
            )
        }
    } else if game_state.viewing_inventory {
        menu_types.push(MenuTypes::InventoryOpen);
        if let Some(character) = focused_character_option {
            let mut equipment_ids = Vec::new();
            let mut consumables_by_type: HashMap<ConsumableTypes, Vec<u32>> = HashMap::new();
            for item in &character.combatant_properties.inventory.items {
                match &item.item_properties {
                    ItemProperties::Consumable(consumable_properties) => {
                        let consumables_of_this_type = consumables_by_type
                            .entry(consumable_properties.consumable_type)
                            .or_insert(Vec::new());
                        consumables_of_this_type.push(item.entity_properties.id)
                    }
                    ItemProperties::Equipment(_) => equipment_ids.push(item.entity_properties.id),
                }
            }
            new_actions = MenuTypes::get_actions(
                &menu_types,
                Some((consumables_by_type, equipment_ids)),
                None,
                None,
                inventory_is_open,
            );
        }
        //
    } else if game_state.viewing_skill_level_up_menu
        || game_state.viewing_attribute_point_assignment_menu
    {
        menu_types.push(MenuTypes::LevelUpAbilities);
        let mut ability_names =
            get_ability_menu_names(&party, game_state.focused_character_id, None);
        ability_names.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        new_actions = MenuTypes::get_actions(
            &menu_types,
            None,
            Some(ability_names),
            None,
            inventory_is_open,
        );
        //
    } else if party.battle_id.is_none() {
        menu_types.push(MenuTypes::OutOfCombat);
        let mut ability_names = get_ability_menu_names(
            &party,
            game_state.focused_character_id,
            Some(AbilityUsableContext::InCombat),
        );
        ability_names.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        if party.current_room.treasure_chest.is_some() {
            menu_types.push(MenuTypes::UnopenedChest);
        }
        if party.current_room.items.len() > 0 {
            menu_types.push(MenuTypes::ItemsOnGround);
        }
        if party.current_room.room_type == DungeonRoomTypes::Stairs {
            menu_types.push(MenuTypes::Staircase)
        }
        new_actions = MenuTypes::get_actions(
            &menu_types,
            None,
            Some(ability_names),
            None,
            inventory_is_open,
        );
    } else {
        menu_types.push(MenuTypes::InCombat);
        let mut ability_names = get_ability_menu_names(
            &party,
            game_state.focused_character_id,
            Some(AbilityUsableContext::OutOfCombat),
        );
        ability_names.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        new_actions = MenuTypes::get_actions(
            &menu_types,
            None,
            Some(ability_names),
            None,
            inventory_is_open,
        );
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
                if &ability
                    .ability_name
                    .get_attributes()
                    .combat_action_properties
                    .usability_context
                    != excluded_context
                {
                    ability_names.push(ability_name.clone());
                }
            }
        }
    }
    ability_names
}
