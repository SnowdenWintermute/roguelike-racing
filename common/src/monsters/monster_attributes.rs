use super::monster_types::MonsterTypes;
use crate::combatants::combat_attributes::CombatAttributes;
use std::collections::HashMap;

impl MonsterTypes {
    pub fn get_per_level_attributes(&self) -> HashMap<CombatAttributes, f32> {
        match self {
            MonsterTypes::Zombie => HashMap::from([
                (CombatAttributes::Damage, 4.0),
                (CombatAttributes::Strength, 3.0),
                (CombatAttributes::Dexterity, 1.0),
                (CombatAttributes::Vitality, 2.0),
                (CombatAttributes::Hp, 2.0),
                (CombatAttributes::Agility, 0.5),
            ]),
            MonsterTypes::SkeletonArcher => HashMap::from([
                (CombatAttributes::Damage, 3.0),
                (CombatAttributes::Dexterity, 3.0),
                (CombatAttributes::Vitality, 1.0),
                (CombatAttributes::Hp, 1.5),
                (CombatAttributes::Agility, 1.5),
            ]),
            MonsterTypes::Scavenger => HashMap::from([
                (CombatAttributes::Damage, 3.0),
                (CombatAttributes::Dexterity, 2.0),
                (CombatAttributes::Strength, 2.0),
                (CombatAttributes::Vitality, 1.5),
                (CombatAttributes::Hp, 1.5),
                (CombatAttributes::Agility, 1.5),
            ]),
            MonsterTypes::Vulture => HashMap::from([
                (CombatAttributes::Damage, 3.0),
                (CombatAttributes::Dexterity, 2.5),
                (CombatAttributes::Strength, 1.5),
                (CombatAttributes::Vitality, 1.5),
                (CombatAttributes::Hp, 1.5),
                (CombatAttributes::Agility, 1.5),
            ]),
            MonsterTypes::FireMage => HashMap::from([
                (CombatAttributes::Intelligence, 4.0),
                (CombatAttributes::Focus, 2.0),
                (CombatAttributes::Vitality, 1.5),
                (CombatAttributes::Hp, 1.5),
                (CombatAttributes::Resilience, 2.0),
                (CombatAttributes::Agility, 1.5),
            ]),
            MonsterTypes::Cultist => HashMap::from([
                (CombatAttributes::Intelligence, 4.0),
                (CombatAttributes::Focus, 2.0),
                (CombatAttributes::Vitality, 1.5),
                (CombatAttributes::Hp, 1.5),
                (CombatAttributes::Resilience, 2.0),
                (CombatAttributes::Agility, 1.5),
            ]),
            MonsterTypes::FireElemental => HashMap::from([
                (CombatAttributes::Intelligence, 4.0),
                (CombatAttributes::Focus, 2.0),
                (CombatAttributes::Vitality, 1.0),
                (CombatAttributes::Hp, 1.0),
                (CombatAttributes::Resilience, 2.0),
                (CombatAttributes::Agility, 1.5),
            ]),
            MonsterTypes::IceElemental => HashMap::from([
                (CombatAttributes::Intelligence, 4.0),
                (CombatAttributes::Focus, 2.0),
                (CombatAttributes::Vitality, 1.0),
                (CombatAttributes::Hp, 1.0),
                (CombatAttributes::Resilience, 2.0),
                (CombatAttributes::Agility, 1.5),
            ]),
            MonsterTypes::MetallicGolem => HashMap::from([
                (CombatAttributes::Damage, 3.0),
                (CombatAttributes::Vitality, 2.0),
                (CombatAttributes::ArmorClass, 20.0),
                (CombatAttributes::Hp, 2.5),
                (CombatAttributes::Resilience, 3.0),
                (CombatAttributes::Agility, 1.5),
            ]),
        }
    }
}
