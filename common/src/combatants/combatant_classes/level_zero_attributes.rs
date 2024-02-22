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
                    (CombatAttributes::Strength, 2),
                    (CombatAttributes::Dexterity, 1),
                    (CombatAttributes::Intelligence, 1),
                    (CombatAttributes::Focus, 0),
                    (CombatAttributes::Vitality, 2),
                    (CombatAttributes::Resilience, 1),
                    (CombatAttributes::Agility, 1),
                    (CombatAttributes::Hp, 20),
                    (CombatAttributes::Mp, 2),
                    (CombatAttributes::Accuracy, 75),
                    (CombatAttributes::Speed, 1),
                ]),
            ),
            (
                CombatantClass::Mage,
                HashMap::from([
                    (CombatAttributes::Strength, 0),
                    (CombatAttributes::Dexterity, 0),
                    (CombatAttributes::Intelligence, 2),
                    (CombatAttributes::Focus, 2),
                    (CombatAttributes::Vitality, 1),
                    (CombatAttributes::Resilience, 1),
                    (CombatAttributes::Agility, 1),
                    (CombatAttributes::Hp, 15),
                    (CombatAttributes::Mp, 4),
                    (CombatAttributes::Accuracy, 65),
                    (CombatAttributes::Speed, 1),
                ]),
            ),
            (
                CombatantClass::Rogue,
                HashMap::from([
                    (CombatAttributes::Strength, 1),
                    (CombatAttributes::Dexterity, 2),
                    (CombatAttributes::Intelligence, 1),
                    (CombatAttributes::Focus, 0),
                    (CombatAttributes::Vitality, 1),
                    (CombatAttributes::Resilience, 1),
                    (CombatAttributes::Agility, 1),
                    (CombatAttributes::Hp, 17),
                    (CombatAttributes::Mp, 3),
                    (CombatAttributes::Accuracy, 85),
                    (CombatAttributes::Speed, 1),
                ]),
            ),
        ])
    });
