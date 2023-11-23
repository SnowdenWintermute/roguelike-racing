use super::Character;
use crate::items::equipment::EquipmentSlots;

impl Character {
    pub fn unequip_slots(
        &mut self,
        slots_to_unequip: &Vec<EquipmentSlots>,
        is_due_to_equipment_swap: bool,
    ) -> Vec<u32> {
        let mut unequipped_item_options = Vec::new();
        for slot in slots_to_unequip {
            unequipped_item_options.push(self.combatant_properties.equipment.remove(&slot))
        }
        let mut ids_of_unequipped_items = Vec::new();
        for item_option in unequipped_item_options {
            if let Some(item) = item_option {
                ids_of_unequipped_items.push(item.entity_properties.id);
                self.inventory.items.push(item);
            }
        }

        if !is_due_to_equipment_swap {
            self.combatant_properties.clamp_curr_hp_to_max();
        }

        ids_of_unequipped_items
    }
}
