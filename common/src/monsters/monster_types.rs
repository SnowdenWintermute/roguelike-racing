use crate::combatants::combatant_classes::CombatantClass;
use core::fmt;
use rand::Rng;
use std::fmt::Display;

pub enum MonsterTypes {
    MetallicGolem,  // High AC
    Zombie,         // 1.25 blunt, .75 slashing, .5 piercing, high HP and vit
    SkeletonArcher, // 1.25 blunt, .75 slashing, .5 piercing, high dex, uses ranged attack
    Scavenger,      // medium hp, .5 blunt, 1.25 slashing, 1 piercing
    Vulture,        // medium hp, .5 blunt, 1.0 slashing, 1.25 piercing
    FireMage,       // low AC and HP, casts fire
    Cultist,        // Low AC and HP, casts cure
    FireElemental,  // .25 damage from physical, casts fire, weak to ice
    IceElemental,   // .25 damage from physical, casts ice, weak to fire
}

impl Display for MonsterTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MonsterTypes::MetallicGolem => "Metallic Golem",
                MonsterTypes::Zombie => "Zombie",
                MonsterTypes::SkeletonArcher => "SkeletonArcher",
                MonsterTypes::Scavenger => "Scavenger",
                MonsterTypes::Vulture => "Vulture",
                MonsterTypes::FireMage => "Fire Mage",
                MonsterTypes::Cultist => "Cultist",
                MonsterTypes::FireElemental => "Fire Elemental",
                MonsterTypes::IceElemental => "Ice Elemental",
            }
        )
    }
}

impl MonsterTypes {
    pub fn select_random() -> Self {
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(1..=100);
        match roll {
            1..=15 => Self::Zombie,
            16..=30 => Self::Scavenger,
            31..=45 => Self::SkeletonArcher,
            46..=60 => Self::Vulture,
            61..=70 => Self::FireMage,
            71..=80 => Self::Cultist,
            81..=85 => Self::FireElemental,
            86..=90 => Self::FireElemental,
            _ => Self::MetallicGolem,
        }
    }

    pub fn get_combatant_class(&self) -> CombatantClass {
        match self {
            MonsterTypes::Zombie => CombatantClass::Warrior,
            MonsterTypes::SkeletonArcher => CombatantClass::Rogue,
            MonsterTypes::Scavenger => CombatantClass::Rogue,
            MonsterTypes::Vulture => CombatantClass::Warrior,
            MonsterTypes::FireMage => CombatantClass::Mage,
            MonsterTypes::Cultist => CombatantClass::Mage,
            MonsterTypes::FireElemental => CombatantClass::Mage,
            MonsterTypes::IceElemental => CombatantClass::Mage,
            MonsterTypes::MetallicGolem => CombatantClass::Warrior,
        }
    }
}
