use super::monster_types::MonsterTypes;
use strum::IntoEnumIterator;

impl MonsterTypes {
    pub fn get_spawnable_types_on_floor(level: u8) -> Vec<MonsterTypes> {
        match level {
            1 => vec![MonsterTypes::Zombie, MonsterTypes::Scavenger],
            2 => vec![
                MonsterTypes::Zombie,
                MonsterTypes::Scavenger,
                MonsterTypes::SkeletonArcher,
                MonsterTypes::Vulture,
            ],
            3 => vec![
                MonsterTypes::Zombie,
                MonsterTypes::Scavenger,
                MonsterTypes::SkeletonArcher,
                MonsterTypes::Vulture,
                MonsterTypes::FireElemental,
                MonsterTypes::IceElemental,
            ],
            _ => MonsterTypes::iter().collect(),
        }
    }
}
