use super::monster_types::MonsterTypes;
use crate::combatants::combat_attributes::CombatAttributes;
use std::collections::HashMap;

impl MonsterTypes {
    pub fn get_starting_attributes(&self) -> HashMap<CombatAttributes, f32> {
        match self {
            MonsterTypes::Zombie => HashMap::from([
                (CombatAttributes::Damage, 0.0),
                (CombatAttributes::Strength, 30.0),
                (CombatAttributes::Dexterity, 1.0),
                (CombatAttributes::Vitality, 2.0),
                (CombatAttributes::Hp, 15.0),
                (CombatAttributes::Agility, 0.5),
                (CombatAttributes::Accuracy, 60.0),
                (CombatAttributes::Speed, 1.0),
            ]),
            MonsterTypes::SkeletonArcher => HashMap::from([
                (CombatAttributes::Damage, 0.0),
                (CombatAttributes::Dexterity, 3.0),
                (CombatAttributes::Vitality, 1.0),
                (CombatAttributes::Hp, 10.0),
                (CombatAttributes::Accuracy, 75.0),
            ]),
            MonsterTypes::Scavenger => HashMap::from([
                (CombatAttributes::Damage, 0.0),
                (CombatAttributes::Dexterity, 2.0),
                (CombatAttributes::Strength, 20.0),
                (CombatAttributes::Vitality, 1.5),
                (CombatAttributes::Hp, 10.5),
                (CombatAttributes::Agility, 1.5),
                (CombatAttributes::Accuracy, 80.0),
            ]),
            MonsterTypes::Vulture => HashMap::from([
                (CombatAttributes::Damage, 0.0),
                (CombatAttributes::Dexterity, 2.5),
                (CombatAttributes::Strength, 20.5),
                (CombatAttributes::Vitality, 1.5),
                (CombatAttributes::Hp, 10.5),
                (CombatAttributes::Accuracy, 80.0),
            ]),
            MonsterTypes::FireMage => HashMap::from([
                (CombatAttributes::Damage, 1.0),
                (CombatAttributes::Intelligence, 4.0),
                (CombatAttributes::Focus, 2.0),
                (CombatAttributes::Vitality, 1.5),
                (CombatAttributes::Hp, 10.5),
                (CombatAttributes::Resilience, 2.0),
                (CombatAttributes::Accuracy, 60.0),
                (CombatAttributes::Speed, 1.0),
            ]),
            MonsterTypes::Cultist => HashMap::from([
                (CombatAttributes::Damage, 1.0),
                (CombatAttributes::Intelligence, 4.0),
                (CombatAttributes::Focus, 2.0),
                (CombatAttributes::Vitality, 1.5),
                (CombatAttributes::Hp, 10.5),
                (CombatAttributes::Resilience, 2.0),
                (CombatAttributes::Accuracy, 60.0),
            ]),
            MonsterTypes::FireElemental => HashMap::from([
                (CombatAttributes::Damage, 2.0),
                (CombatAttributes::Intelligence, 4.0),
                (CombatAttributes::Focus, 2.0),
                (CombatAttributes::Vitality, 1.0),
                (CombatAttributes::Hp, 10.0),
                (CombatAttributes::Resilience, 2.0),
                (CombatAttributes::Accuracy, 60.0),
            ]),
            MonsterTypes::IceElemental => HashMap::from([
                (CombatAttributes::Damage, 2.0),
                (CombatAttributes::Intelligence, 4.0),
                (CombatAttributes::Focus, 2.0),
                (CombatAttributes::Vitality, 1.0),
                (CombatAttributes::Hp, 10.0),
                (CombatAttributes::Resilience, 2.0),
                (CombatAttributes::Accuracy, 60.0),
            ]),
            MonsterTypes::MetallicGolem => HashMap::from([
                (CombatAttributes::Damage, 3.0),
                (CombatAttributes::Strength, 30.0),
                (CombatAttributes::Vitality, 2.0),
                (CombatAttributes::ArmorClass, 15.0),
                (CombatAttributes::Hp, 17.5),
                (CombatAttributes::Resilience, 3.0),
                (CombatAttributes::Accuracy, 70.0),
            ]),
        }
    }
}
