use super::monster_types::MonsterTypes;
use crate::combatants::abilities::CombatantAbility;
use crate::combatants::abilities::CombatantAbilityNames;
use std::collections::HashMap;

impl MonsterTypes {
    pub fn get_abilities(&self) -> HashMap<CombatantAbilityNames, CombatantAbility> {
        let mut abilities = HashMap::new();
        abilities.insert(
            CombatantAbilityNames::Attack,
            CombatantAbility::create_by_name(&CombatantAbilityNames::Attack),
        );
        abilities.insert(
            CombatantAbilityNames::AttackMeleeMainhand,
            CombatantAbility::create_by_name(&CombatantAbilityNames::AttackMeleeMainhand),
        );
        abilities.insert(
            CombatantAbilityNames::AttackMeleeOffhand,
            CombatantAbility::create_by_name(&CombatantAbilityNames::AttackMeleeOffhand),
        );
        abilities.insert(
            CombatantAbilityNames::AttackRangedMainhand,
            CombatantAbility::create_by_name(&CombatantAbilityNames::AttackRangedMainhand),
        );

        match self {
            MonsterTypes::FireMage => {
                abilities.insert(
                    CombatantAbilityNames::Fire,
                    CombatantAbility::create_by_name(&CombatantAbilityNames::Fire),
                );
            }
            MonsterTypes::FireElemental => {
                abilities.insert(
                    CombatantAbilityNames::Fire,
                    CombatantAbility::create_by_name(&CombatantAbilityNames::Fire),
                );
            }
            MonsterTypes::IceElemental => {
                abilities.insert(
                    CombatantAbilityNames::Ice,
                    CombatantAbility::create_by_name(&CombatantAbilityNames::Ice),
                );
            }
            MonsterTypes::Cultist => {
                abilities.insert(
                    CombatantAbilityNames::Healing,
                    CombatantAbility::create_by_name(&CombatantAbilityNames::Healing),
                );
            }
            _ => (),
        };

        abilities
    }
}
