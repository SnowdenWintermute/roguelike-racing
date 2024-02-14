#![allow(dead_code)]
mod random_monster_names;
// use self::random_monster_names::MONSTER_FIRST_NAMES;
use self::random_monster_names::MONSTER_LAST_NAMES;
use crate::combat::magical_elements::MagicalElements;
use crate::combatants::abilities::CombatantAbility;
use crate::combatants::abilities::CombatantAbilityNames;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::combatants::combatant_traits::CombatantTraits;
use crate::combatants::CombatantClass;
use crate::combatants::CombatantControlledBy;
use crate::combatants::CombatantProperties;
use crate::game::id_generator::IdGenerator;
use crate::primatives::EntityProperties;
use rand::seq::SliceRandom;
use rand::Rng;
use rand_distr::Distribution;
use rand_distr::StandardNormal;
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Monster {
    pub entity_properties: EntityProperties,
    pub combatant_properties: CombatantProperties,
}

pub fn generate_random_monster_name() -> String {
    let mut rng = rand::thread_rng();
    // let first_name = MONSTER_FIRST_NAMES.choose(&mut rng).unwrap();
    let last_name = MONSTER_LAST_NAMES.choose(&mut rng).unwrap();

    format!("{}", last_name).to_string()
}

impl Monster {
    pub fn generate(id_generator: &mut IdGenerator, level: u8, base_hp: u16) -> Monster {
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
        let mut rng = rand::thread_rng();
        let modified_hp = {
            let half_of_base_hp = base_hp as f32 * 0.5;
            // CHAT GPT HOW TO GET RANDOM NORMALLY DISTRIBUTED VALUE IN RANGE
            let normal = StandardNormal;
            let min = base_hp as f32 - half_of_base_hp;
            let max = base_hp as f32 + half_of_base_hp;
            let mut value: f32 = normal.sample(&mut rng);
            value *= (min - max) / 2.0;
            value += (min + max) / 2.0;
            value
        };
        inherent_attributes.insert(CombatAttributes::Hp, modified_hp as u16);
        inherent_attributes.insert(CombatAttributes::Damage, 1);
        inherent_attributes.insert(CombatAttributes::Strength, 2 * level as u16);
        inherent_attributes.insert(CombatAttributes::Dexterity, 1 * level as u16);
        inherent_attributes.insert(CombatAttributes::Vitality, 1 * level as u16);
        inherent_attributes.insert(CombatAttributes::Intelligence, 1 * level as u16);
        inherent_attributes.insert(CombatAttributes::Focus, 1 * level as u16);
        inherent_attributes.insert(CombatAttributes::Resilience, 1 + 1 * level as u16);
        inherent_attributes.insert(CombatAttributes::ArmorClass, 10 * (level - 1) as u16);
        inherent_attributes.insert(
            CombatAttributes::Agility,
            cmp::max(1, 1 * (level as u16 / 4)),
        );
        inherent_attributes.insert(CombatAttributes::Accuracy, 75);

        let trait_randomizer_number = rng.gen_range(0.0..1.0);
        if trait_randomizer_number <= 0.25 {
            monster
                .combatant_properties
                .traits
                .push(CombatantTraits::Undead);
            monster.entity_properties.name = format!("undead {}", monster.entity_properties.name);
        } else if trait_randomizer_number >= 0.25 && trait_randomizer_number < 0.35 {
            monster
                .combatant_properties
                .traits
                .push(CombatantTraits::ElementalAffinityPercent(
                    MagicalElements::Fire,
                    200,
                ));
            monster.entity_properties.name = format!("fire {}", monster.entity_properties.name);
        } else if trait_randomizer_number >= 0.35 && trait_randomizer_number < 0.55 {
            monster
                .combatant_properties
                .traits
                .push(CombatantTraits::ElementalAffinityPercent(
                    MagicalElements::Fire,
                    -100,
                ));
            monster.entity_properties.name = format!("ice {}", monster.entity_properties.name);
        }

        let monster_randomizer_number = rng.gen_range(0.0..1.0);
        if monster_randomizer_number < 0.33 {
            inherent_attributes.insert(CombatAttributes::Agility, cmp::max(1, 1 * level as u16));
            inherent_attributes.insert(CombatAttributes::Dexterity, 2 * level as u16);
            monster.entity_properties.name = format!("agile {}", monster.entity_properties.name);
        } else if monster_randomizer_number > 0.33 && monster_randomizer_number < 0.66 {
            inherent_attributes.insert(CombatAttributes::Strength, 3 * level as u16);
            inherent_attributes.insert(CombatAttributes::Vitality, 2 * level as u16);
            inherent_attributes.insert(CombatAttributes::ArmorClass, 20 * (level - 1) as u16);
            monster.entity_properties.name = format!("strong {}", monster.entity_properties.name);
        } else if monster_randomizer_number > 0.66 {
            inherent_attributes.insert(CombatAttributes::Intelligence, 2 * level as u16);
            inherent_attributes.insert(CombatAttributes::Focus, 2 * level as u16);
            inherent_attributes.insert(CombatAttributes::Resilience, 1 + 2 * level as u16);
            monster.entity_properties.name = format!("magical {}", monster.entity_properties.name);
        }

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
            CombatantAbility::create_by_name(&CombatantAbilityNames::Attack),
        );

        monster
    }
}
