#![allow(dead_code, unused)]

use common::combatants::{abilities::CombatantAbilities, CombatAttributes};
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
    SelectInventorySlot(u16),
    OpenTreasureChest,
    TakeItem,
    // Item Selected
    UseItem,
    DropItem,
    ShardItem,
    // InCombat
    Attack,
    UseAbility(CombatantAbilities),
    LevelUpAbility(CombatantAbilities),
    SetAssignAttributePointsMenuOpen(bool),
    AssignAttributePoint(CombatAttributes),
}

impl MenuTypes {
    pub fn get_menu(
        menu_types: Vec<MenuTypes>,
        num_items_in_inventory: Option<u16>,
        all_abilities: Option<Vec<CombatantAbilities>>,
        combat_useable_abilities: Option<Vec<CombatantAbilities>>,
        non_combat_usable_abilities: Option<Vec<CombatantAbilities>>,
    ) -> Vec<GameActions> {
        let mut menu_items: Vec<GameActions> = Vec::new();

        for menu_type in menu_types {
            match menu_type {
                MenuTypes::OutOfCombat => {
                    menu_items.push(GameActions::ToggleReadyToExplore);
                    menu_items.push(GameActions::UseAutoinjector);
                    menu_items.push(GameActions::ToggleInventoryOpen);
                    if let Some(abilities) = non_combat_usable_abilities.clone() {
                        for ability in abilities {
                            menu_items.push(GameActions::UseAbility(ability))
                        }
                    }
                    menu_items.push(GameActions::SetAssignAttributePointsMenuOpen(true))
                }
                MenuTypes::UnopenedChest => menu_items.push(GameActions::OpenTreasureChest),
                MenuTypes::ItemsOnGround => menu_items.push(GameActions::TakeItem),
                MenuTypes::InCombat => {
                    menu_items.push(GameActions::Attack);
                    menu_items.push(GameActions::UseAutoinjector);
                    if let Some(abilities) = combat_useable_abilities.clone() {
                        for ability in abilities {
                            menu_items.push(GameActions::UseAbility(ability))
                        }
                    }
                    menu_items.push(GameActions::ToggleInventoryOpen);
                }
                MenuTypes::LevelUpAbilities => {
                    menu_items.push(GameActions::SetAssignAttributePointsMenuOpen(true));
                    if let Some(abilities) = all_abilities.clone() {
                        for ability in abilities {
                            menu_items.push(GameActions::LevelUpAbility(ability))
                        }
                    }
                }
                MenuTypes::InventoryOpen => {
                    menu_items.push(GameActions::ToggleInventoryOpen);
                    if let Some(num) = num_items_in_inventory {
                        for i in 0..=num {
                            menu_items.push(GameActions::SelectInventorySlot(i))
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
