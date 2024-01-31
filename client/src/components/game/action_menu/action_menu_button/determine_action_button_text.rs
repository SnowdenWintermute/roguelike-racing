use crate::components::game::action_menu::enums::GameActions;
use crate::store::game_store::GameStore;
use common::game::getters::get_party;
use std::rc::Rc;

pub fn determine_action_button_text(action: GameActions, game_state: Rc<GameStore>) -> String {
    match action {
        GameActions::ToggleReadyToExplore => {
            let party = game_state
                .get_current_party()
                .expect("to be in a party when showing this button");
            match party.current_room.room_type {
                common::dungeon_rooms::DungeonRoomTypes::Stairs => "Vote to stay".to_string(),
                _ => "Ready to explore".to_string(),
            }
        }
        GameActions::SetInventoryOpen(open_status) => {
            if open_status {
                "Open inventory".to_string()
            } else {
                "Close inventory".to_string()
            }
        }
        GameActions::ToggleInventoryOpen => {
            if game_state.viewing_inventory {
                "Close Inventory".to_string()
            } else {
                "Open Inventory".to_string()
            }
        }
        GameActions::ToggleViewingEquipedItems => {
            if game_state.viewing_equipped_items {
                "View Unequipped Items".to_string()
            } else {
                "View Equipment".to_string()
            }
        }
        GameActions::SelectItem(id) => determine_select_item_text(&id, game_state),
        GameActions::OpenTreasureChest => "Open treasure chest".to_string(),
        GameActions::TakeItem => "Pick up item".to_string(),
        GameActions::UseItem(id) => determine_use_item_text(&id, game_state).to_string(),
        GameActions::DropItem(_) => "Drop".to_string(),
        GameActions::DeselectItem => "Cancel".to_string(),
        GameActions::ShardItem(_) => "Convert to shard".to_string(),
        GameActions::SelectAbility(name) => format!("{}", name),
        GameActions::LevelUpAbility(_name) => "Level up ability".to_string(),
        GameActions::SetAssignAttributePointsMenuOpen(_open_status) => {
            "Assign attributes".to_string()
        }
        GameActions::AssignAttributePoint(_attribute) => "Increase attribute".to_string(),
        GameActions::CycleAbilityTargets(direction)
        | GameActions::CycleConsumableTargets(direction) => match direction {
            common::primatives::NextOrPrevious::Next => "Next target".to_string(),
            common::primatives::NextOrPrevious::Previous => "Prev target".to_string(),
        },
        GameActions::CycleAbilityTargetingScheme | GameActions::CycleConsumableTargetingScheme => {
            "Targeting scheme".to_string()
        }
        GameActions::DeselectAbility | GameActions::DeselectConsumable => "Cancel".to_string(),
        GameActions::UseSelectedAbility => "Execute".to_string(),
        GameActions::ToggleReadyToDescend => "Vote to descend".to_string(),
    }
}

fn determine_use_item_text<'a>(id: &u32, game_state: Rc<GameStore>) -> &'a str {
    let party_id = game_state
        .current_party_id
        .expect("only call this fn if char is in a party");
    let game = game_state.game.as_ref().expect("");
    let party = get_party(&game, party_id).expect("");
    let character = party
        .characters
        .get(&game_state.focused_character_id)
        .expect("");

    for (_, item) in &character.combatant_properties.equipment {
        if item.entity_properties.id == *id {
            return "Unequip";
        }
    }

    for item in &character.combatant_properties.inventory.items {
        if item.entity_properties.id == *id {
            match item.item_properties {
                common::items::ItemProperties::Consumable(_) => return "Use",
                common::items::ItemProperties::Equipment(_) => return "Equip",
            }
        }
    }
    "No item found"
}

fn determine_select_item_text(id: &u32, game_state: Rc<GameStore>) -> String {
    let party_id = game_state
        .current_party_id
        .expect("only call this fn if char is in a party");
    let game = game_state.game.as_ref().expect("");
    let party = get_party(&game, party_id).expect("");
    let character = party
        .characters
        .get(&game_state.focused_character_id)
        .expect("");

    for (_, item) in &character.combatant_properties.equipment {
        if item.entity_properties.id == *id {
            return item.entity_properties.name.clone();
        }
    }

    for item in &character.combatant_properties.inventory.items {
        if item.entity_properties.id == *id {
            return item.entity_properties.name.clone();
        }
    }
    "No item found".to_string()
}
