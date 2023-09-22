use crate::errors::AppError;
use crate::items::{EquipmentSlots, Item, ItemCategories};

use super::open_treasure_chest::open_treasure_chest;
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
    ClearConsumableAndAbilitySelections,
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
    pub fn process_player_input(
        &mut self,
        party_id: u32,
        player_character_id: u32,
        player_input: PlayerInputs,
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

        let ref mut current_room = adventuring_party.current_room;
        let player_is_active = match adventuring_party.active_player_id {
            Some(active_player_id) => active_player_id == player_character.entity_properties.id,
            None => false,
        };

        match player_input {
            PlayerInputs::SelectConsumable(inventory_slot) => {
                let selected_consumable = player_character
                    .inventory
                    .items
                    .get(inventory_slot as usize)
                    .ok_or(AppError {
                        error_type: crate::errors::AppErrorTypes::InvalidInput,
                        message: "Tried to select an item but no item found in the inventory slot"
                            .to_string(),
                    })?;

                if selected_consumable.item_category != ItemCategories::Consumable {
                    return Err(AppError {
                        error_type: crate::errors::AppErrorTypes::InvalidInput,
                        message: "Can't select a non-consumable item".to_string(),
                    });
                }
                player_character.combatant_properties.selected_item_slot = Some(inventory_slot);
            }
            PlayerInputs::UseSelectedConsumable => {
                let selected_item_slot = player_character
                    .combatant_properties
                    .selected_item_slot
                    .ok_or(AppError {
                    error_type: crate::errors::AppErrorTypes::InvalidInput,
                    message: "Tried to use the selected item but no item was selected".to_string(),
                })?;
                let selected_consumable = player_character
                    .inventory
                    .items
                    .get(selected_item_slot as usize)
                    .ok_or(AppError {
                        error_type: crate::errors::AppErrorTypes::InvalidInput,
                        message: "Tried to select an item but no item found in the inventory slot"
                            .to_string(),
                    })?;
                // based on targets and consumable type, do something complicated
            }
            PlayerInputs::OpenTreasureChest => {
                open_treasure_chest(&mut self.id_generator, &adventuring_party, current_room)?
            }
            _ => (),
        }

        Ok(())
    }
}
