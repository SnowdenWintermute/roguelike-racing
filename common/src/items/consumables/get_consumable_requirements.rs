use super::ConsumableTypes;
use crate::combatants::combat_attributes::CombatAttributes;
use std::collections::HashMap;

impl ConsumableTypes {
    pub fn get_requirements(&self, item_level: u8) -> Option<HashMap<CombatAttributes, u8>> {
        match self {
            ConsumableTypes::HpAutoinjector => None,
            ConsumableTypes::Grenade => Some(HashMap::from([(
                CombatAttributes::Intelligence,
                item_level,
            )])),
            ConsumableTypes::SmokeBomb => {
                Some(HashMap::from([(CombatAttributes::Dexterity, item_level)]))
            }
        }
    }
}
