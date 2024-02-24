#![allow(dead_code)]
mod monster_attributes;
mod monster_equipment;
mod monster_traits;
mod monster_types;
mod random_monster_names;
use self::monster_types::MonsterTypes;
use crate::combatants::abilities::CombatantAbility;
use crate::combatants::abilities::CombatantAbilityNames;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::combatants::combatant_classes::CombatantClass;
use crate::combatants::CombatantControlledBy;
use crate::combatants::CombatantProperties;
use crate::combatants::ExperiencePoints;
use crate::game::id_generator::IdGenerator;
use crate::primatives::EntityProperties;
use rand_distr::Distribution;
use rand_distr::Normal;
use serde::Deserialize;
use serde::Serialize;
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
        // roll a random monster type from list of pre determined types
        let monster_type = MonsterTypes::select_random();
        // create a monster of that type
        let mut monster = Monster::new(
            id_generator.get_next_entity_id(),
            format!("{}", monster_type),
        );
        // - some combatant class
        let monster_class = monster_type.get_combatant_class();
        monster.combatant_properties.combatant_class = monster_class;
        // - assign level
        monster.combatant_properties.level = level;
        // - assign their "discretionary" attributes
        // - assign attributes that would have come from wearing gear
        let attributes_per_level = monster_type.get_per_level_attributes();
        for (attribute, value) in &attributes_per_level {
            let current = monster
                .combatant_properties
                .inherent_attributes
                .entry(*attribute)
                .or_insert(0);
            *current += *value as u16;
        }
        // - randomize their hp a little
        let base_hp = monster
            .combatant_properties
            .inherent_attributes
            .get(&CombatAttributes::Hp)
            .unwrap_or_else(|| &0);
        let mut rng = rand::thread_rng();
        let modified_hp = {
            let normal = Normal::new(*base_hp as f32, 3.0).expect("");
            let v = normal.sample(&mut rng);
            v
        };
        monster
            .combatant_properties
            .inherent_attributes
            .insert(CombatAttributes::Hp, modified_hp as u16);
        // - assign traits
        monster.combatant_properties.traits = monster_type.get_traits();
        // - equip weapons
        // - assign their abilities

        monster.combatant_properties.abilities.insert(
            CombatantAbilityNames::Attack,
            CombatantAbility::create_by_name(&CombatantAbilityNames::Attack),
        );
        monster.combatant_properties.abilities.insert(
            CombatantAbilityNames::AttackMeleeMainhand,
            CombatantAbility::create_by_name(&CombatantAbilityNames::AttackMeleeMainhand),
        );
        monster.combatant_properties.abilities.insert(
            CombatantAbilityNames::AttackMeleeOffhand,
            CombatantAbility::create_by_name(&CombatantAbilityNames::AttackMeleeOffhand),
        );
        monster.combatant_properties.abilities.insert(
            CombatantAbilityNames::AttackRangedMainhand,
            CombatantAbility::create_by_name(&CombatantAbilityNames::AttackRangedMainhand),
        );

        monster
    }

    pub fn new(id: u32, name: String) -> Self {
        let mut to_return = Monster {
            entity_properties: EntityProperties { id, name },
            combatant_properties: CombatantProperties::new(
                &CombatantClass::Warrior,
                HashMap::new(),
                CombatantControlledBy::AI,
            ),
        };
        to_return.combatant_properties.experience_points = ExperiencePoints {
            current: 0,
            required_for_next_level: None,
        };
        to_return
    }
}
