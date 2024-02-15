use super::combatant_traits::CombatantTraits;
use super::CombatantProperties;
use crate::combat::magical_elements::MagicalElements;
use std::collections::HashMap;

impl CombatantProperties {
    pub fn get_total_elemental_affinites(&self) -> HashMap<MagicalElements, i16> {
        let mut totals = HashMap::new();
        for (affinity, value) in &self.inherent_elemental_affinities {
            totals.insert(affinity.clone(), *value);
        }

        for combatant_trait in &self.traits {
            match combatant_trait {
                CombatantTraits::ElementalAffinityPercent(element, value) => {
                    let current = totals.entry(element.clone()).or_insert(0);
                    *current += value
                }
                _ => (),
            }
        }

        totals
    }
}
