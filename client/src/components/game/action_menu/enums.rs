#![allow(unused, dead_code)]
use common::combatants::abilities::CombatantAbilityNames;
use common::combatants::combat_attributes::CombatAttributes;
use common::primatives::NextOrPrevious;

pub enum MenuTypes {
    InCombat,
    AbilitySelected,
    ConsumableSelected,
    OutOfCombat,
    LevelUpAbilities,
    AttributePointAssignment,
    InventoryOpen,
    ViewingEquipedItems,
    ItemSelected(u32),
    ItemsOnGround,
    UnopenedChest,
    Staircase,
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum GameActions {
    ToggleReadyToExplore,
    ToggleReadyToDescend,
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
    DeselectConsumable,
    // InCombat
    SelectAbility(CombatantAbilityNames),
    DeselectAbility,
    CycleTargets(NextOrPrevious),
    CycleTargetingScheme,
    UseSelectedAbility,
    LevelUpAbility(CombatantAbilityNames),
    SetAssignAttributePointsMenuOpen(bool),
    AssignAttributePoint(CombatAttributes),
}
