#![allow(dead_code, unused)]

use common::{
    combatants::{abilities::CombatantAbilityNames, CombatAttributes},
    items::Item,
    primatives::NextOrPrevious,
};
use gloo::console::log;
pub enum MenuTypes {
    InCombat,
    AbilitySelected,
    OutOfCombat,
    LevelUpAbilities,
    AttributePointAssignment,
    InventoryOpen,
    ViewingEquipedItems,
    ItemSelected(u32),
    ItemsOnGround,
    UnopenedChest,
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum GameActions {
    ToggleReadyToExplore,
    SetInventoryOpen(bool),
    ToggleInventoryOpen,
    ToggleViewingEquipedItems,
    UseAutoinjector,
    SelectItem(u32),
    OpenTreasureChest,
    TakeItem,
    // Item Selected
    UseItem(u32),
    DropItem(u32),
    ShardItem(u32),
    DeselectItem,
    // InCombat
    Attack,
    SelectAbility(CombatantAbilityNames),
    DeselectAbility,
    CycleTargets(NextOrPrevious),
    CycleTargetingScheme,
    UseSelectedAbility,
    LevelUpAbility(CombatantAbilityNames),
    SetAssignAttributePointsMenuOpen(bool),
    AssignAttributePoint(CombatAttributes),
}

impl MenuTypes {
    pub fn get_menu(
        menu_types: &Vec<MenuTypes>,
        item_ids: Option<Vec<u32>>,
        abilities: Option<Vec<CombatantAbilityNames>>,
    ) -> Vec<GameActions> {
        let mut menu_items: Vec<GameActions> = Vec::new();

        for menu_type in menu_types {
            match menu_type {
                MenuTypes::OutOfCombat => {
                    menu_items.push(GameActions::ToggleInventoryOpen);
                    menu_items.push(GameActions::ToggleReadyToExplore);
                    menu_items.push(GameActions::UseAutoinjector);
                    add_abilities_to_menu(&abilities, &mut menu_items);
                    menu_items.push(GameActions::SetAssignAttributePointsMenuOpen(true))
                }
                MenuTypes::UnopenedChest => menu_items.push(GameActions::OpenTreasureChest),
                MenuTypes::ItemsOnGround => menu_items.push(GameActions::TakeItem),
                MenuTypes::InCombat => {
                    add_abilities_to_menu(&abilities, &mut menu_items);
                    menu_items.push(GameActions::UseAutoinjector);
                    menu_items.push(GameActions::ToggleInventoryOpen);
                }
                MenuTypes::AbilitySelected => {
                    menu_items.push(GameActions::DeselectAbility);
                    menu_items.push(GameActions::CycleTargets(NextOrPrevious::Next));
                    menu_items.push(GameActions::CycleTargets(NextOrPrevious::Previous));
                    menu_items.push(GameActions::UseSelectedAbility);
                    menu_items.push(GameActions::CycleTargetingScheme);
                }
                MenuTypes::LevelUpAbilities => {
                    menu_items.push(GameActions::SetAssignAttributePointsMenuOpen(true));
                    add_abilities_to_menu(&abilities, &mut menu_items);
                }
                MenuTypes::InventoryOpen => {
                    menu_items.push(GameActions::ToggleInventoryOpen);
                    menu_items.push(GameActions::ToggleViewingEquipedItems);
                    if let Some(item_ids) = &item_ids {
                        for id in item_ids {
                            menu_items.push(GameActions::SelectItem(*id))
                        }
                    }
                }
                MenuTypes::ViewingEquipedItems => {
                    menu_items.push(GameActions::ToggleInventoryOpen);
                    menu_items.push(GameActions::ToggleViewingEquipedItems);
                    if let Some(item_ids) = &item_ids {
                        for id in item_ids {
                            menu_items.push(GameActions::SelectItem(*id))
                        }
                    }
                }
                MenuTypes::ItemSelected(id) => {
                    menu_items.push(GameActions::DeselectItem);
                    menu_items.push(GameActions::UseItem(*id));
                    menu_items.push(GameActions::ShardItem(*id));
                    menu_items.push(GameActions::DropItem(*id));
                }
                MenuTypes::AttributePointAssignment => {
                    menu_items.push(GameActions::SetAssignAttributePointsMenuOpen(false));
                }
            }
        }

        menu_items
    }
}

fn add_abilities_to_menu(
    ability_names: &Option<Vec<CombatantAbilityNames>>,
    menu_items: &mut Vec<GameActions>,
) {
    if let Some(names) = ability_names.clone() {
        for ability_name in names {
            menu_items.push(GameActions::SelectAbility(ability_name))
        }
    }
}
