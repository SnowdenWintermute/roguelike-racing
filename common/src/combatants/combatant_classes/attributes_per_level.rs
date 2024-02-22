use super::CombatantClass;
use crate::combatants::combat_attributes::CombatAttributes;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static ATTRIBUTES_BY_LEVEL: Lazy<HashMap<CombatantClass, HashMap<CombatAttributes, f32>>> =
    Lazy::new(|| {
        HashMap::from([
            (
                CombatantClass::Warrior,
                HashMap::from([
                    (CombatAttributes::Strength, 2.5),
                    (CombatAttributes::Dexterity, 1.5),
                    (CombatAttributes::Intelligence, 1.0),
                    (CombatAttributes::Focus, 0.0),
                    (CombatAttributes::Vitality, 2.0),
                    (CombatAttributes::Resilience, 1.0),
                    (CombatAttributes::Agility, 1.0),
                    (CombatAttributes::Hp, 2.0),
                    (CombatAttributes::Mp, 1.0),
                ]),
            ),
            (
                CombatantClass::Mage,
                HashMap::from([
                    (CombatAttributes::Strength, 0.5),
                    (CombatAttributes::Dexterity, 0.5),
                    (CombatAttributes::Intelligence, 2.5),
                    (CombatAttributes::Focus, 2.0),
                    (CombatAttributes::Vitality, 1.0),
                    (CombatAttributes::Resilience, 1.5),
                    (CombatAttributes::Agility, 1.0),
                    (CombatAttributes::Hp, 1.0),
                    (CombatAttributes::Mp, 2.0),
                ]),
            ),
            (
                CombatantClass::Rogue,
                HashMap::from([
                    (CombatAttributes::Strength, 1.5),
                    (CombatAttributes::Dexterity, 2.0),
                    (CombatAttributes::Intelligence, 1.0),
                    (CombatAttributes::Focus, 0.0),
                    (CombatAttributes::Vitality, 1.5),
                    (CombatAttributes::Resilience, 1.5),
                    (CombatAttributes::Agility, 1.5),
                    (CombatAttributes::Hp, 1.5),
                    (CombatAttributes::Mp, 1.5),
                ]),
            ),
        ])
    });

