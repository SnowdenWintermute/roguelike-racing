use super::CharacterAndItem;
use super::CharacterAndSlot;
use crate::combat::combat_actions::CombatActionTarget;
use crate::combatants::abilities::CombatantAbilityNames;
use crate::combatants::CombatantClass;
use crate::items::equipment::EquipmentSlots;
use serde::Deserialize;
use serde::Serialize;

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
    DeleteCharacter(u32),
    ToggleReady,
    // use items and abilities
    UseSelectedConsumable,
    SelectAbility(ClientSelectAbilityPacket),
    SelectConsumable(ClientSelectConsumablePacket),
    UseSelectedAbility(u32),
    ChangeAbilityTargets(ChangeTargetsPacket),
    ChangeConsumableTargets(ChangeTargetsPacket),
    ClearConsumableAndAbilitySelections,
    // manage equipment and items
    UnequipEquipmentSlot(CharacterAndSlot),
    ShardInventorySlot(u8),
    EquipInventoryItem(EquipItemRequest),
    // manage abilities
    LevelUpAbilitySlot(u8),
    // exploration
    ToggleReadyToExplore,
    ToggleReadyToGoDownStairs,
    // treasure chests / monster loot
    PickTreasureChestLock,
    DisarmTrappedChest,
    OpenTreasureChest,
    TakeItemOnGround(CharacterAndItem),
    AcknowledgeReceiptOfItemOnGroundUpdate(u32),
    DropItem(CharacterAndItem),
    EquipItemOnGround(u32),
    DropEquippedItem(CharacterAndSlot),
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
pub struct EquipItemRequest {
    pub character_id: u32,
    pub item_id: u32,
    pub alt_slot: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnequipSlotRequest {
    pub character_id: u32,
    pub slot: EquipmentSlots,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientSelectAbilityPacket {
    pub character_id: u32,
    pub ability_name_option: Option<CombatantAbilityNames>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientSelectConsumablePacket {
    pub character_id: u32,
    pub consumable_id_option: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChangeTargetsPacket {
    pub character_id: u32,
    pub new_targets: CombatActionTarget,
}
