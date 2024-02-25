use super::CombatantClass;
use crate::combatants::combat_attributes::CombatAttributes;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static BASE_STARTING_ATTRIBUTES: Lazy<HashMap<CombatantClass, HashMap<CombatAttributes, u8>>> =
    Lazy::new(|| {
        HashMap::from([
            (
                CombatantClass::Warrior,
                HashMap::from([
                    (CombatAttributes::Strength, 3),
                    (CombatAttributes::Dexterity, 1),
                    (CombatAttributes::Vitality, 1),
                    (CombatAttributes::Hp, 20),
                    (CombatAttributes::Mp, 2),
                    (CombatAttributes::Accuracy, 75),
                    (CombatAttributes::Speed, 1),
                ]),
            ),
            (
                CombatantClass::Mage,
                HashMap::from([
                    (CombatAttributes::Intelligence, 3),
                    (CombatAttributes::Focus, 2),
                    (CombatAttributes::Hp, 15),
                    (CombatAttributes::Mp, 4),
                    (CombatAttributes::Accuracy, 65),
                    (CombatAttributes::Speed, 1),
                ]),
            ),
            (
                CombatantClass::Rogue,
                HashMap::from([
                    (CombatAttributes::Strength, 2),
                    (CombatAttributes::Dexterity, 3),
                    (CombatAttributes::Hp, 17),
                    (CombatAttributes::Mp, 3),
                    (CombatAttributes::Accuracy, 85),
                    (CombatAttributes::Speed, 1),
                ]),
            ),
        ])
    });
