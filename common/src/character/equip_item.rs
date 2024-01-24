use super::Character;
use crate::app_consts::error_messages;
use crate::errors::AppError;
use crate::items::equipment::EquipmentSlots;

impl Character {
    pub fn slot_item_is_equipped(&self, item_id: &u32) -> Option<EquipmentSlots> {
        for (slot, equipped_item) in &self.combatant_properties.equipment {
            if &equipped_item.entity_properties.id == item_id {
                return Some(slot.clone());
            }
        }
        return None;
    }

    /// returns list of ids of any items which were unequipped
    pub fn equip_item(&mut self, item_id: u32, alt_slot: bool) -> Result<Vec<u32>, AppError> {
        let mut item_and_index_option = None;
        for (i, item_in_inventory) in self.inventory.items.iter().enumerate() {
            if item_in_inventory.entity_properties.id == item_id {
                item_and_index_option = Some((item_in_inventory, i));
                break;
            }
        }

        let item_and_index = match item_and_index_option {
            Some(item) => item,
            None => {
                return Err(AppError {
                    error_type: crate::errors::AppErrorTypes::InvalidInput,
                    message: error_messages::INVALID_ITEM_ID.to_string(),
                });
            }
        };

        let (item, item_inventory_index) = item_and_index;

        if !self.combatant_properties.can_use_item(&item) {
            return Err(AppError {
                error_type: crate::errors::AppErrorTypes::InvalidInput,
                message: error_messages::ITEM_REQUIREMENTS_NOT_MET.to_string(),
            });
        }
        // @TODO: Check if equiping the item would necessitate unequiping multiple items,
        // (as with equiping a 2h weapon when wielding two 1h items) and
        // if so, check if there is space in the inventory to accomodate unequiping those
        // items. Reject if not.

        let equipment_properties = item.get_equipment_properties()?;

        let possible_slots = equipment_properties.get_equippable_slots();
        let slot = match alt_slot {
            true => {
                if let Some(alternate_slot) = possible_slots.alternate {
                    alternate_slot
                } else {
                    possible_slots.main
                }
            }
            false => possible_slots.main,
        };

        let slots_to_unequip = match slot {
            EquipmentSlots::MainHand => {
                if equipment_properties.is_two_handed() {
                    vec![EquipmentSlots::MainHand, EquipmentSlots::OffHand]
                } else {
                    vec![slot.clone()]
                }
            }
            EquipmentSlots::OffHand => {
                if let Some(equipment_in_main_hand) = self
                    .combatant_properties
                    .equipment
                    .get(&EquipmentSlots::MainHand)
                {
                    if equipment_in_main_hand
                        .get_equipment_properties()?
                        .is_two_handed()
                    {
                        vec![EquipmentSlots::MainHand, EquipmentSlots::OffHand]
                    } else {
                        vec![slot.clone()]
                    }
                } else {
                    vec![slot.clone()]
                }
            }
            _ => vec![slot.clone()],
        };

        let ids_of_unequipped_items = self.unequip_slots(&slots_to_unequip, true);
        // remove item to equip from inventory
        let item_to_equip = self.inventory.items.remove(item_inventory_index);
        // add newly equipped item to equipment hashmap
        self.combatant_properties
            .equipment
            .insert(slot, item_to_equip);

        self.combatant_properties.clamp_curr_hp_to_max();

        Ok(ids_of_unequipped_items)
    }
}
