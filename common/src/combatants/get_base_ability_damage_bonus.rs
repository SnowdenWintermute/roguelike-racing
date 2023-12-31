use super::{combat_attributes::CombatAttributes, CombatantProperties};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug)]
pub enum DamageSource {
    Melee,
    Ranged,
    Magical,
}

impl CombatantProperties {
    pub fn get_base_ability_damage_bonus(
        total_attributes: &HashMap<CombatAttributes, u16>,
        damage_source: DamageSource,
    ) -> u16 {
        let base_damage = total_attributes
            .get(&CombatAttributes::Damage)
            .unwrap_or_else(|| &0);
        let bonus_stat = match damage_source {
            DamageSource::Melee => total_attributes
                .get(&CombatAttributes::Strength)
                .unwrap_or_else(|| &0),
            DamageSource::Ranged => total_attributes
                .get(&CombatAttributes::Dexterity)
                .unwrap_or_else(|| &0),
            DamageSource::Magical => total_attributes
                .get(&CombatAttributes::Intelligence)
                .unwrap_or_else(|| &0),
        };

        bonus_stat + base_damage
    }
}
