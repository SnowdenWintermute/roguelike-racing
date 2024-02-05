#![allow(dead_code)]
mod random_monster_names;
use self::random_monster_names::MONSTER_FIRST_NAMES;
use self::random_monster_names::MONSTER_LAST_NAMES;
use crate::combatants::abilities::CombatantAbility;
use crate::combatants::abilities::CombatantAbilityNames;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::combatants::CombatantClass;
use crate::combatants::CombatantControlledBy;
use crate::combatants::CombatantProperties;
use crate::game::id_generator::IdGenerator;
use crate::primatives::EntityProperties;
use rand::seq::SliceRandom;
use serde::Deserialize;
use serde::Serialize;
use std::cmp;
use std::collections::HashMap;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq)]
pub enum MonsterTraits {
    ManaShield,
    HpRegen,
    AcidicSkin,
    AbrasiveArmor,
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq)]
pub enum MonsterAbilities {
    PoisonSting,
    AcidicStrike,
    HeatLance,
    Thorns,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Monster {
    pub entity_properties: EntityProperties,
    pub combatant_properties: CombatantProperties,
}

pub fn generate_random_monster_name() -> String {
    let mut rng = rand::thread_rng();
    let first_name = MONSTER_FIRST_NAMES.choose(&mut rng).unwrap();
    let last_name = MONSTER_LAST_NAMES.choose(&mut rng).unwrap();

    format!("{} {}", first_name, last_name).to_string()
}

impl Monster {
    pub fn generate(id_generator: &mut IdGenerator, level: u8, hp: u16) -> Monster {
        let mut monster = Monster {
            entity_properties: EntityProperties {
                id: id_generator.get_next_entity_id(),
                name: generate_random_monster_name().to_string(),
            },
            combatant_properties: CombatantProperties::new(
                &CombatantClass::None,
                HashMap::new(),
                CombatantControlledBy::AI,
            ),
        };

        let inherent_attributes = &mut monster.combatant_properties.inherent_attributes;
        inherent_attributes.insert(CombatAttributes::Hp, hp);
        inherent_attributes.insert(CombatAttributes::Damage, 1);
        inherent_attributes.insert(CombatAttributes::Strength, 3 * level as u16);
        inherent_attributes.insert(CombatAttributes::Dexterity, 1 * level as u16);
        inherent_attributes.insert(CombatAttributes::Vitality, 1 * level as u16);
        inherent_attributes.insert(CombatAttributes::Resilience, 1 * level as u16);
        inherent_attributes.insert(CombatAttributes::ArmorClass, 10 * (level - 1) as u16);
        inherent_attributes.insert(
            CombatAttributes::Agility,
            cmp::max(1, 1 * (level as u16 / 4)),
        );
        inherent_attributes.insert(CombatAttributes::Accuracy, 75);

        let total_attributes = monster.combatant_properties.get_total_attributes();
        let max_hp_option = total_attributes.get(&CombatAttributes::Hp);
        if let Some(max_hp) = max_hp_option {
            monster.combatant_properties.hit_points = *max_hp
        }
        let max_mana_option = total_attributes.get(&CombatAttributes::Mp);
        if let Some(max_mana) = max_mana_option {
            monster.combatant_properties.mana = *max_mana
        }

        monster.combatant_properties.abilities.insert(
            CombatantAbilityNames::Attack,
            CombatantAbility::new(CombatantAbilityNames::Attack, None),
        );

        monster
    }
}
