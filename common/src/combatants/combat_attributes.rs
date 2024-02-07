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
    Damage,           // flat bonus to physical damage
    ArmorPenetration, // negates armor class
    Accuracy,         // chance to hit with physical attacks and abilities
    // defensive
    ArmorClass, // compared with final damage of physical attack, reduces damage on a curve
    Evasion,    // dodge an attack
    Hp,         // if reduced to 0 or below, combatant is "dead"
    // utility
    Speed, // determines turn order
    Mp,    // a resource for ability use
    // MAIN
    // offensive
    Focus,     // negates %magic reduction and increases spell crit chance and crit multiplier
    Dexterity, // ranged damage, accuracy, physical crit chance, armor ranged armor pen
    Intelligence, // mp, magic ability damage
    Strength,  // damage with melee attacks, melee crit multiplier, melee armor pen
    // defensive
    Vitality,   // hp, and debuff duration
    Resilience, // %magic damage reduction, healing received, debuff duration
    Agility,    // movement speed, evasion, physical crit chance reduction
}

pub const CORE_ATTRIBUTES: [CombatAttributes; 4] = [
    CombatAttributes::Dexterity,
    CombatAttributes::Intelligence,
    CombatAttributes::Strength,
    CombatAttributes::Vitality,
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
