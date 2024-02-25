use super::CharacterAndDirection;
use super::CharacterAndItem;
use super::CharacterAndSlot;
use super::CharacterId;
use crate::combat::combat_actions::CombatAction;
use crate::combat::combat_actions::CombatActionTarget;
use crate::combatants::abilities::CombatantAbilityNames;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::combatants::combatant_classes::CombatantClass;
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
    DeleteCharacter(CharacterId),
    ToggleReady,
    // use items and abilities
    UseSelectedCombatAction(CharacterId), // character_id
    SelectCombatAction(CharacterAndCombatAction),
    CycleCombatActionTargets(CharacterAndDirection),
    CycleCombatActionTargetingSchemes(CharacterId),
    // manage equipment and items
    UnequipEquipmentSlot(CharacterAndSlot),
    ShardInventorySlot(u8),
    EquipInventoryItem(EquipItemRequest),
    // manage abilities
    LevelUpAbilitySlot(u8),
    IncrementAttribute(CharacterId, CombatAttributes),
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
pub struct CharacterAndCombatAction {
    pub character_id: u32,
    pub combat_action_option: Option<CombatAction>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChangeTargetsPacket {
    pub character_id: u32,
    pub new_targets: CombatActionTarget,
}
