use crate::dungeon_rooms::DungeonRoomTypes;
use crate::errors::AppError;
use crate::items::EquipmentSlots;

use super::Game;

pub struct PlayerActionRequest {
    party_id: u32,
    player_character_id: u32,
    // action: PlayerActions,
}

pub enum PlayerInputs {
    // use items and abilities
    SelectConsumable(u8),
    UseSelectedConsumable,
    SelectAbilitySlot(u8),
    UseSelectedAbility,
    ChangeTargetIds(Vec<u8>),
    // manage equipment and items
    UnequipEquipmentSlot(EquipmentSlots),
    ShardInventorySlot(u8),
    EquipInventoryItem(u8, EquipmentSlots),
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

impl Game {
    pub fn calculate_valid_player_input_context(
        &mut self,
        party_id: u32,
        player_character_id: u32,
    ) -> Result<(), AppError> {
        let adventuring_party = self.adventuring_parties.get(&party_id).ok_or(AppError {
            error_type: crate::errors::AppErrorTypes::InvalidInput,
            message: "tried to process player input but couldn't find their party".to_string(),
        })?;

        let player_character = adventuring_party
            .player_characters
            .get(&player_character_id)
            .ok_or(AppError {
                error_type: crate::errors::AppErrorTypes::InvalidInput,
                message: "tried to process player input but couldn't find the player character"
                    .to_string(),
            })?;

        let room_type = adventuring_party.current_room.room_type;
        let player_is_active = match adventuring_party.active_player_id {
            Some(active_player_id) => active_player_id == player_character.entity_properties.id,
            None => false,
        };

        Ok(())
    }
}
