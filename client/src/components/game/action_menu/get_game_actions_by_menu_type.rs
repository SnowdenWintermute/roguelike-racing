use super::enums::GameActions;
use super::enums::MenuTypes;
use common::combat::combat_actions::CombatAction;
use common::combatants::abilities::CombatantAbilityNames;
use common::items::consumables::ConsumableTypes;
use common::primatives::NextOrPrevious;
use std::collections::HashMap;

impl MenuTypes {
    pub fn get_actions(
        menu_types: &Vec<MenuTypes>,
        item_ids: Option<(HashMap<ConsumableTypes, Vec<u32>>, Vec<u32>)>,
        abilities: Option<Vec<CombatantAbilityNames>>,
    ) -> Vec<GameActions> {
        let mut menu_items: Vec<GameActions> = Vec::new();

        for menu_type in menu_types {
            match menu_type {
                MenuTypes::OutOfCombat => {
                    menu_items.push(GameActions::ToggleInventoryOpen);
                    menu_items.push(GameActions::ToggleReadyToExplore);
                    add_abilities_to_menu(&abilities, &mut menu_items);
                    menu_items.push(GameActions::SetAssignAttributePointsMenuOpen(true))
                }
                MenuTypes::UnopenedChest => menu_items.push(GameActions::OpenTreasureChest),
                MenuTypes::ItemsOnGround => menu_items.push(GameActions::TakeItem),
                MenuTypes::InCombat => {
                    add_abilities_to_menu(&abilities, &mut menu_items);
                    menu_items.push(GameActions::ToggleInventoryOpen);
                }
                MenuTypes::CombatActionSelected => {
                    menu_items.push(GameActions::DeselectCombatAction);
                    menu_items.push(GameActions::CycleTargets(NextOrPrevious::Next));
                    menu_items.push(GameActions::CycleTargets(NextOrPrevious::Previous));
                    menu_items.push(GameActions::UseSelectedCombatAction);
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
                        for (consumable_type, ids) in &item_ids.0 {
                            menu_items.push(GameActions::SelectItem(ids[0], ids.len() as u16))
                        }
                        for id in &item_ids.1 {
                            menu_items.push(GameActions::SelectItem(*id, 1))
                        }
                    }
                }
                MenuTypes::ViewingEquipedItems => {
                    menu_items.push(GameActions::ToggleInventoryOpen);
                    menu_items.push(GameActions::ToggleViewingEquipedItems);
                    if let Some(item_ids) = &item_ids {
                        for id in &item_ids.1 {
                            menu_items.push(GameActions::SelectItem(*id, 1))
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
                MenuTypes::Staircase => menu_items.push(GameActions::ToggleReadyToDescend),
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
            menu_items.push(GameActions::SelectCombatAction(CombatAction::AbilityUsed(
                ability_name,
            )))
        }
    }
}
