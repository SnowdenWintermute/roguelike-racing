use super::monster_types::MonsterTypes;
use crate::combat::hp_change_source_types::PhysicalDamageTypes;
use crate::combat::magical_elements::MagicalElements;
use crate::combatants::combatant_traits::CombatantTraits;

impl MonsterTypes {
    pub fn get_traits(&self) -> Vec<CombatantTraits> {
        match self {
            MonsterTypes::Zombie => vec![
                CombatantTraits::Undead,
                CombatantTraits::PhysicalDamageTypeResistancePercent(
                    PhysicalDamageTypes::Blunt,
                    -25,
                ),
                CombatantTraits::PhysicalDamageTypeResistancePercent(
                    PhysicalDamageTypes::Slashing,
                    25,
                ),
                CombatantTraits::PhysicalDamageTypeResistancePercent(
                    PhysicalDamageTypes::Piercing,
                    50,
                ),
            ],
            MonsterTypes::SkeletonArcher => vec![
                CombatantTraits::Undead,
                CombatantTraits::PhysicalDamageTypeResistancePercent(
                    PhysicalDamageTypes::Blunt,
                    -25,
                ),
                CombatantTraits::PhysicalDamageTypeResistancePercent(
                    PhysicalDamageTypes::Slashing,
                    25,
                ),
                CombatantTraits::PhysicalDamageTypeResistancePercent(
                    PhysicalDamageTypes::Piercing,
                    50,
                ),
            ],
            MonsterTypes::Scavenger => vec![
                CombatantTraits::PhysicalDamageTypeResistancePercent(
                    PhysicalDamageTypes::Blunt,
                    50,
                ),
                CombatantTraits::PhysicalDamageTypeResistancePercent(
                    PhysicalDamageTypes::Slashing,
                    -25,
                ),
            ],
            MonsterTypes::Vulture => vec![
                CombatantTraits::PhysicalDamageTypeResistancePercent(
                    PhysicalDamageTypes::Blunt,
                    50,
                ),
                CombatantTraits::PhysicalDamageTypeResistancePercent(
                    PhysicalDamageTypes::Piercing,
                    -25,
                ),
            ],
            MonsterTypes::FireMage => vec![CombatantTraits::ElementalAffinityPercent(
                MagicalElements::Fire,
                50,
            )],
            MonsterTypes::Cultist => vec![],
            MonsterTypes::FireElemental => vec![
                CombatantTraits::ElementalAffinityPercent(MagicalElements::Fire, 200),
                CombatantTraits::ElementalAffinityPercent(MagicalElements::Ice, -100),
                CombatantTraits::PhysicalDamageTypeResistancePercent(
                    PhysicalDamageTypes::Blunt,
                    50,
                ),
                CombatantTraits::PhysicalDamageTypeResistancePercent(
                    PhysicalDamageTypes::Piercing,
                    50,
                ),
                CombatantTraits::PhysicalDamageTypeResistancePercent(
                    PhysicalDamageTypes::Slashing,
                    50,
                ),
            ],
            MonsterTypes::IceElemental => vec![
                CombatantTraits::ElementalAffinityPercent(MagicalElements::Ice, 200),
                CombatantTraits::ElementalAffinityPercent(MagicalElements::Fire, -100),
                CombatantTraits::PhysicalDamageTypeResistancePercent(
                    PhysicalDamageTypes::Blunt,
                    50,
                ),
                CombatantTraits::PhysicalDamageTypeResistancePercent(
                    PhysicalDamageTypes::Piercing,
                    50,
                ),
                CombatantTraits::PhysicalDamageTypeResistancePercent(
                    PhysicalDamageTypes::Slashing,
                    50,
                ),
            ],
            MonsterTypes::MetallicGolem => vec![],
        }
    }
}
