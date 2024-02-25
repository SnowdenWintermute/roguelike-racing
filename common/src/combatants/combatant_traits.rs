use core::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::combat::hp_change_source_types::PhysicalDamageTypes;
use crate::combat::magical_elements::MagicalElements;
use serde::Deserialize;
use serde::Serialize;

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum CombatantTraits {
    HpBioavailabilityPercent(u8),
    MpBioavailabilityPercent(u8),
    ElementalAffinityPercent(MagicalElements, i16),
    Undead,
    PhysicalDamageTypeResistancePercent(PhysicalDamageTypes, i16),
}

impl Display for CombatantTraits {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            CombatantTraits::HpBioavailabilityPercent(percentage) => {
                write!(f, "Hp Bioavailability {percentage}%")
            }
            CombatantTraits::MpBioavailabilityPercent(percentage) => {
                write!(f, "Mp Bioavailability {percentage}%")
            }
            CombatantTraits::ElementalAffinityPercent(element, percentage) => {
                write!(f, "{element} affinity {percentage}%")
            }
            CombatantTraits::Undead => write!(f, "Undead"),
            CombatantTraits::PhysicalDamageTypeResistancePercent(damage_type, percentage) => {
                write!(f, "{damage_type} affinity {percentage}%")
            }
        }
    }
}

impl CombatantTraits {
    pub fn get_description(&self) -> &str {
        match self{
            CombatantTraits::HpBioavailabilityPercent(_) => "Effectiveness of HP Autoinjectors",
            CombatantTraits::MpBioavailabilityPercent(_) => "Effectiveness of MP Autoinjectors",
            CombatantTraits::ElementalAffinityPercent(_, _) => "Resistance or weakness to this element. If above 100%, actions of this element will cause healing instead of damage.",
            CombatantTraits::Undead => "Healing magic damages this target",
            CombatantTraits::PhysicalDamageTypeResistancePercent(_, _) => "Resistance or weakness to this damage type",
        }
    }
}
