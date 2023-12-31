use core::fmt;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(
    Debug, EnumIter, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize, PartialOrd, Ord,
)]
pub enum CombatAttributes {
    Damage,           // flat bonus to physical damage
    ArmorPenetration, // negates armor class
    ArmorClass,       // compared with final damage of physical attack, reduces damage on a curve
    Accuracy,         // chance to hit with physical attacks and abilities
    Evasion,          // dodge an attack
    Focus,            // magical accuracy
    Obscurity,        // chance to fully resist a magical ability
    Speed,            // determines turn order
    Hp,               // if reduced to 0 or below, combatant is "dead"
    Mp,               // a resource for ability use
    Dexterity,        // ranged damage, accuracy with physical attacks, physical crit chance, armor
    // penetration for ranged piercing weapons
    Intelligence, // mp, magic ability damage, obscurity, focus
    Strength,     // damage with melee attacks, melee crit multiplier, melee armor penetration for
    // piercing and slashing weapons
    Vitality,   // hp and %damage reduction after AC damage reduction
    Resilience, // %magic damage reduction, healing received, magical crit damage reduction
    Agility,    // movement speed, evasion, physical crit damage reduction
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
            CombatAttributes::Obscurity => write!(f, "Obscurity"),
            CombatAttributes::Speed => write!(f, "Speed"),
            CombatAttributes::Hp => write!(f, "HP"),
            CombatAttributes::Mp => write!(f, "MP"),
            CombatAttributes::ArmorPenetration => write!(f, "Armor Pen."),
        }
    }
}
