#![allow(dead_code, unused)]

use common::{
    combatants::{abilities::CombatantAbilityNames, CombatAttributes},
    items::Item,
};
pub enum MenuTypes {
    InCombat,
    OutOfCombat,
    LevelUpAbilities,
    AttributePointAssignment,
    InventoryOpen,
    ItemSelected,
    ItemsOnGround,
    UnopenedChest,
}

#[derive(Hash, Eq, PartialEq)]
pub enum GameActions {
    ToggleReadyToExplore,
    SetInventoryOpen(bool),
    ToggleInventoryOpen,
    UseAutoinjector,
    SelectItem(u32),
    OpenTreasureChest,
    TakeItem,
    // Item Selected
    UseItem,
    DropItem,
    ShardItem,
    // InCombat
    Attack,
    UseAbility(CombatantAbilityNames),
    LevelUpAbility(CombatantAbilityNames),
    SetAssignAttributePointsMenuOpen(bool),
    AssignAttributePoint(CombatAttributes),
}

impl MenuTypes {
    pub fn get_menu(
        menu_types: &Vec<MenuTypes>,
        item_ids_in_inventory: Option<Vec<u32>>,
        abilities: Option<Vec<CombatantAbilityNames>>,
    ) -> Vec<GameActions> {
        let mut menu_items: Vec<GameActions> = Vec::new();

        for menu_type in menu_types {
            match menu_type {
                MenuTypes::OutOfCombat => {
                    menu_items.push(GameActions::ToggleReadyToExplore);
                    menu_items.push(GameActions::UseAutoinjector);
                    menu_items.push(GameActions::ToggleInventoryOpen);
                    add_abilities_to_menu(&abilities, &mut menu_items);
                    menu_items.push(GameActions::SetAssignAttributePointsMenuOpen(true))
                }
                MenuTypes::UnopenedChest => menu_items.push(GameActions::OpenTreasureChest),
                MenuTypes::ItemsOnGround => menu_items.push(GameActions::TakeItem),
                MenuTypes::InCombat => {
                    menu_items.push(GameActions::Attack);
                    menu_items.push(GameActions::UseAutoinjector);
                    add_abilities_to_menu(&abilities, &mut menu_items);
                    menu_items.push(GameActions::ToggleInventoryOpen);
                }
                MenuTypes::LevelUpAbilities => {
                    menu_items.push(GameActions::SetAssignAttributePointsMenuOpen(true));
                    add_abilities_to_menu(&abilities, &mut menu_items);
                }
                MenuTypes::InventoryOpen => {
                    menu_items.push(GameActions::ToggleInventoryOpen);
                    if let Some(item_ids) = &item_ids_in_inventory {
                        for id in item_ids {
                            menu_items.push(GameActions::SelectItem(*id))
                        }
                    }
                }
                MenuTypes::ItemSelected => {
                    menu_items.push(GameActions::UseItem);
                    menu_items.push(GameActions::ShardItem);
                    menu_items.push(GameActions::DropItem);
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
            menu_items.push(GameActions::UseAbility(ability_name))
        }
    }
}
