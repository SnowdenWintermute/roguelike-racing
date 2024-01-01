use super::CombatantProperties;
use crate::items::{
    equipment::{EquipmentProperties, EquipmentSlots},
    ItemProperties,
};

impl CombatantProperties {
    pub fn get_equipped_item(&self, slot: &EquipmentSlots) -> Option<&EquipmentProperties> {
        match self.equipment.get(slot) {
            Some(item) => match &item.item_properties {
                ItemProperties::Consumable(_) => None,
                ItemProperties::Equipment(properties) => Some(properties),
            },
            None => None,
        }
    }
}
