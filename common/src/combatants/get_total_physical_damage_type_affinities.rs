use super::combatant_traits::CombatantTraits;
use super::CombatantProperties;
use crate::combat::hp_change_source_types::PhysicalDamageTypes;
use std::collections::HashMap;

impl CombatantProperties {
    pub fn get_total_physical_damage_type_affinites(
        &self,
    ) -> HashMap<PhysicalDamageTypes, i16> {
        let mut totals = HashMap::new();

        for combatant_trait in &self.traits {
            match combatant_trait {
                CombatantTraits::PhysicalDamageTypeResistancePercent(damage_type, percentage) => {
                    let current = totals.entry(damage_type.clone()).or_insert(0);
                    *current += percentage
                }
                _ => (),
            }
        }

        totals
    }
}
