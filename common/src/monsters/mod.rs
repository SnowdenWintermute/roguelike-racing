#![allow(dead_code)]
mod monster_abilities;
mod monster_equipment;
mod monster_per_level_attributes;
mod monster_spawnable_floors;
mod monster_starting_attributes;
mod monster_traits;
pub mod monster_types;
mod random_monster_names;
use self::monster_types::MonsterTypes;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::combatants::combatant_classes::CombatantClass;
use crate::combatants::combatant_species::CombatantSpecies;
use crate::combatants::CombatantControlledBy;
use crate::combatants::CombatantProperties;
use crate::combatants::ExperiencePoints;
use crate::game::id_generator::IdGenerator;
use crate::primatives::EntityProperties;
use rand::seq::SliceRandom;
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
    pub monster_type: MonsterTypes,
}

impl Monster {
    pub fn generate(id_generator: &mut IdGenerator, level: u8) -> Monster {
        // roll a random monster type from list of pre determined types
        // let monster_type = MonsterTypes::select_random();
        let spawnable_types = MonsterTypes::get_spawnable_types_on_floor(level);
        let mut rng = rand::thread_rng();
        let monster_type = spawnable_types.choose(&mut rng).unwrap();
        let combatant_species = match monster_type {
            MonsterTypes::MetallicGolem => todo!(),
            MonsterTypes::Zombie => CombatantSpecies::Skeleton,
            MonsterTypes::SkeletonArcher => CombatantSpecies::Skeleton,
            MonsterTypes::Scavenger => CombatantSpecies::Velociraptor,
            MonsterTypes::Vulture => todo!(),
            MonsterTypes::FireMage => todo!(),
            MonsterTypes::Cultist => todo!(),
            MonsterTypes::FireElemental => todo!(),
            MonsterTypes::IceElemental => todo!(),
        };
        // @TODO - Performance - don't send the name, derive it on the client from the species
        // create a monster of that type
        let mut monster = Monster::new(
            id_generator.get_next_entity_id(),
            format!("{}", monster_type),
            monster_type.clone(),
            combatant_species,
        );
        // - some combatant class
        let monster_class = monster_type.get_combatant_class();
        monster.combatant_properties.combatant_class = monster_class;
        // - assign level
        monster.combatant_properties.level = level;
        // - assign their "discretionary" attributes
        // - assign attributes that would have come from wearing gear
        let starting_attributes = monster_type.get_starting_attributes();
        for (attribute, value) in &starting_attributes {
            let current = monster
                .combatant_properties
                .inherent_attributes
                .entry(*attribute)
                .or_insert(0);
            *current += *value as u16;
        }
        let attributes_per_level = monster_type.get_per_level_attributes();
        for (attribute, value) in &attributes_per_level {
            let current = monster
                .combatant_properties
                .inherent_attributes
                .entry(*attribute)
                .or_insert(0);
            *current += *value as u16 * (level - 1) as u16;
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
        // set hp and mp to max
        monster.combatant_properties.set_hp_and_mp_to_max();
        // - equip weapons
        monster.combatant_properties.equipment = monster_type.get_equipment(id_generator);
        // - assign their abilities

        monster.combatant_properties.abilities = monster_type.get_abilities();

        monster
    }

    pub fn new(
        id: u32,
        name: String,
        monster_type: MonsterTypes,
        combatant_species: CombatantSpecies,
    ) -> Self {
        let mut to_return = Monster {
            entity_properties: EntityProperties { id, name },
            combatant_properties: CombatantProperties::new(
                &CombatantClass::Warrior,
                &combatant_species,
                HashMap::new(),
                CombatantControlledBy::AI,
            ),
            monster_type,
        };
        to_return.combatant_properties.experience_points = ExperiencePoints {
            current: 0,
            required_for_next_level: None,
        };
        to_return
    }
}
