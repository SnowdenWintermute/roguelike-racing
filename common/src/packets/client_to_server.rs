use crate::{character::combatant_properties::CombatantClass, items::EquipmentSlots};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PlayerInputs {
    // lobby
    RequestGameList,
    CreateGame(GameCreation),
    JoinGame(String),
    LeaveGame,
    CreateAdventuringParty(String),
    JoinAdventuringParty(u32),
    LeaveAdventuringParty,
    CreateCharacter(CharacterCreation),
    ChangeCharacterClass(CharacterClassSelection),
    DeselectCharacter,
    ToggleReady,
    // use items and abilities
    SelectConsumable(u8),
    UseSelectedConsumable,
    SelectAbilitySlot(u8),
    UseSelectedAbility,
    ChangeTargetIds(Vec<u8>),
    ClearConsumableAndAbilitySelections,
    // manage equipment and items
    UnequipEquipmentSlot(EquipmentSlots),
    ShardInventorySlot(u8),
    EquipInventoryItem(EquipItem),
    // manage abilities
    LevelUpAbilitySlot(u8),
    // exploration
    ToggleReadyToExplore,
    ToggleReadyToGoDownStairs,
    // treasure chests / monster loot
    PickTreasureChestLock,
    DisarmTrappedChest,
    OpenTreasureChest,
    TakeItemOnGround,
    EquipItemOnGround,
    ShardItemOnGround,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerInputRequest {
    pub party_id: u32,
    pub player_character_id: u32,
    pub player_input: PlayerInputs,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameCreation {
    pub name: String,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CharacterCreation {
    pub character_name: String,
    pub combatant_class: CombatantClass,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharacterClassSelection {
    character_id: u32,
    combatant_class: CombatantClass,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EquipItem {
    item_slot: u8,
    equipment_slot: EquipmentSlots,
}
