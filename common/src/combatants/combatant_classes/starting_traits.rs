use super::CombatantClass;
use crate::combatants::combatant_traits::CombatantTraits;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static STARTING_COMBATANT_TRAITS: Lazy<HashMap<CombatantClass, Vec<CombatantTraits>>> =
    Lazy::new(|| {
        HashMap::from([
            (
                CombatantClass::Warrior,
                vec![CombatantTraits::HpBioavailabilityPercent(200)],
            ),
            (
                CombatantClass::Mage,
                vec![CombatantTraits::MpBioavailabilityPercent(200)],
            ),
            (
                CombatantClass::Rogue,
                vec![
                    CombatantTraits::HpBioavailabilityPercent(150),
                    CombatantTraits::MpBioavailabilityPercent(150),
                ],
            ),
        ])
    });
