use crate::items::equipment::EquipmentProperties;
use crate::items::equipment::EquipmentTraits;

impl EquipmentProperties {
    pub fn get_weapon_percent_damage_increase_trait_damage_modifier(&self) -> f32 {
        if let Some(eq_traits) = &self.traits {
            for equipment_trait in eq_traits {
                match equipment_trait {
                    EquipmentTraits::DamagePercentage(percentage) => {
                        return 1.0 + *percentage as f32 / 100.0
                    }
                    _ => continue,
                }
            }
        }
        1.0
    }
}
