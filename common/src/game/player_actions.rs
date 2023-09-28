use super::RoguelikeRacerGame;
use crate::character::combatant_properties::CombatantClass;
use crate::errors::AppError;
use crate::game::player_input_handlers::{
    open_treasure_chest, select_consumable, use_selected_consumable,
};
use crate::items::EquipmentSlots;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerInputRequest {
    pub party_id: u32,
    pub player_character_id: u32,
    pub player_input: PlayerInputs,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameCreation {
    pub name: String,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterCreation {
    name: String,
    class: CombatantClass,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EquipItem {
    item_slot: u8,
    equipment_slot: EquipmentSlots,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PlayerInputs {
    // lobby
    CreateGame(GameCreation),
    JoinGame(String),
    LeaveGame(String),
    SelectCharacter(CharacterCreation),
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

impl RoguelikeRacerGame {
    pub fn process_player_input(
        &mut self,
        player_input_request: PlayerInputRequest,
    ) -> Result<(), AppError> {
        let PlayerInputRequest {
            party_id,
            player_character_id,
            player_input,
        } = player_input_request;

        let adventuring_party = self
            .adventuring_parties
            .get_mut(&party_id)
            .ok_or(AppError {
                error_type: crate::errors::AppErrorTypes::InvalidInput,
                message: "tried to process player input but couldn't find their party".to_string(),
            })?;

        let player_character = adventuring_party
            .player_characters
            .get_mut(&player_character_id)
            .ok_or(AppError {
                error_type: crate::errors::AppErrorTypes::InvalidInput,
                message: "tried to process player input but couldn't find the player character"
                    .to_string(),
            })?;

        match player_input {
            PlayerInputs::SelectConsumable(inventory_slot) => {
                select_consumable::select_consumable(player_character, inventory_slot)?
            }
            PlayerInputs::UseSelectedConsumable => {
                use_selected_consumable::use_selected_consumable(
                    adventuring_party,
                    player_character_id,
                )?
            }
            PlayerInputs::OpenTreasureChest => {
                open_treasure_chest::open_treasure_chest(&mut self.id_generator, adventuring_party)?
            }
            _ => (),
        }

        println!("{:#?}", self);

        Ok(())
    }
}
