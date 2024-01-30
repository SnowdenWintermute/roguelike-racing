use super::enums::GameActions;
use super::enums::MenuTypes;
use common::combatants::abilities::CombatantAbilityNames;
use common::primatives::NextOrPrevious;

impl MenuTypes {
    pub fn get_actions(
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
                    add_abilities_to_menu(&abilities, &mut menu_items);
                    menu_items.push(GameActions::SetAssignAttributePointsMenuOpen(true))
                }
                MenuTypes::UnopenedChest => menu_items.push(GameActions::OpenTreasureChest),
                MenuTypes::ItemsOnGround => menu_items.push(GameActions::TakeItem),
                MenuTypes::InCombat => {
                    add_abilities_to_menu(&abilities, &mut menu_items);
                    menu_items.push(GameActions::ToggleInventoryOpen);
                }
                MenuTypes::AbilitySelected => {
                    menu_items.push(GameActions::DeselectAbility);
                    menu_items.push(GameActions::CycleAbilityTargets(NextOrPrevious::Next));
                    menu_items.push(GameActions::CycleAbilityTargets(NextOrPrevious::Previous));
                    menu_items.push(GameActions::UseSelectedAbility);
                    menu_items.push(GameActions::CycleAbilityTargetingScheme);
                }
                MenuTypes::ConsumableSelected => {
                    menu_items.push(GameActions::DeselectConsumable);
                    menu_items.push(GameActions::CycleConsumableTargets(NextOrPrevious::Next));
                    menu_items.push(GameActions::CycleConsumableTargets(
                        NextOrPrevious::Previous,
                    ));
                    menu_items.push(GameActions::UseSelectedAbility);
                    menu_items.push(GameActions::CycleConsumableTargetingScheme);
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
            menu_items.push(GameActions::SelectAbility(ability_name))
        }
    }
}
