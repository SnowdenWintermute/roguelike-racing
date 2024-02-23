use core::fmt;
use serde::Deserialize;
use serde::Serialize;
use strum_macros::EnumIter;

#[derive(
    Debug, EnumIter, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize, PartialOrd, Ord,
)]
pub enum CombatAttributes {
    // DERIVED
    // offensive
    Damage,           // adds a flat bonus to physical damage
    ArmorPenetration, // subtracted from target's armor class
    Accuracy,         // after target's evasion subtracted, the chance for an evadable actions to
    // hit its target
    // defensive
    ArmorClass, // compared with final damage of physical attack, reduces damage on a curve
    Evasion,    // reduces the chance to be hit by evadable actions
    Hp,         // if 0 or below, a combatant can no longer take actions
    // utility
    Speed, // determines turn order
    Mp,    // a resource for ability use
    // MAIN
    // offensive
    Strength, // damage with melee attacks, melee crit multiplier, melee armor pen, shield block
    // chance
    Dexterity, // ranged damage, accuracy, physical crit chance, armor ranged armor pen, shield
    // block chance
    Intelligence, // mp, magic ability damage
    Focus,        // negates %magic reduction and increases spell crit chance and crit multiplier
    // defensive
    Vitality,   // hp, and debuff duration, shield block damage reduction
    Resilience, // %magic damage reduction, healing received, debuff duration
    Agility,    // movement speed, evasion, physical crit chance reduction
}

pub const CORE_ATTRIBUTES: [CombatAttributes; 4] = [
    CombatAttributes::Dexterity,
    CombatAttributes::Intelligence,
    CombatAttributes::Strength,
    CombatAttributes::Vitality,
];

pub const ATTRIBUTE_POINT_ASSIGNABLE_ATTRIBUTES: [CombatAttributes; 7] = [
    CombatAttributes::Dexterity,
    CombatAttributes::Intelligence,
    CombatAttributes::Strength,
    CombatAttributes::Vitality,
    CombatAttributes::Resilience,
    CombatAttributes::Focus,
    CombatAttributes::Agility,
];

impl fmt::Display for CombatAttributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CombatAttributes::Damage => write!(f, "Damage"),
            CombatAttributes::ArmorClass => write!(f, "Armor Class"),
            CombatAttributes::Dexterity => write!(f, "Dexterity"),
            CombatAttributes::Strength => write!(f, "Strength"),
            CombatAttributes::Intelligence => write!(f, "Intelligence"),
            CombatAttributes::Vitality => write!(f, "Vitality"),
            CombatAttributes::Resilience => write!(f, "Resilience"),
            CombatAttributes::Agility => write!(f, "Agility"),
            CombatAttributes::Accuracy => write!(f, "Accuracy"),
            CombatAttributes::Focus => write!(f, "Focus"),
            CombatAttributes::Evasion => write!(f, "Evasion"),
            CombatAttributes::Speed => write!(f, "Speed"),
            CombatAttributes::Hp => write!(f, "HP"),
            CombatAttributes::Mp => write!(f, "MP"),
            CombatAttributes::ArmorPenetration => write!(f, "Armor Pen."),
        }
    }
}

impl CombatAttributes {
    pub fn get_description(&self) -> &str {
        match self {
            CombatAttributes::Damage => "A flat bonus applied to physical attacks",
            CombatAttributes::ArmorPenetration => "Negates a target's armor class",
            CombatAttributes::Accuracy => "Chance to hit a target with an evadable attack",
            CombatAttributes::ArmorClass => "Reduces physical damage",
            CombatAttributes::Evasion => "Chance to avoid being hit",
            CombatAttributes::Hp => "If reduced to zero, the combatant can no longer take actions",
            CombatAttributes::Speed => "Determines turn order",
            CombatAttributes::Mp => "The primary resource for using abilities",
            CombatAttributes::Focus => "Negates magic defense and increases crit chance and crit multiplier for spells",
            CombatAttributes::Dexterity => "Increases accuracy, crit chance with physical attacks, ranged attack damage and ranged attack armor penetration",
            CombatAttributes::Intelligence => "Increases mana and spell damage",
            CombatAttributes::Strength => "Increases attack damage, crit multiplier and armor penetration with physical attacks",
            CombatAttributes::Vitality => "Increases hit points and reduces physical damage by a percentage",
            CombatAttributes::Resilience => "Reduces magical damage by a percentage and increases healing received from spells",
            CombatAttributes::Agility => "Increases evasion and speed",
        }
    }
}
