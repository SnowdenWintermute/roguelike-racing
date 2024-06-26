use self::outfit_new_character::outfit_new_character;
use crate::combatants::abilities::CombatantAbility;
use crate::combatants::abilities::CombatantAbilityNames;
use crate::combatants::combatant_classes::CombatantClass;
use crate::combatants::combatant_species::CombatantSpecies;
use crate::combatants::CombatantControlledBy;
use crate::combatants::CombatantProperties;
use crate::game::RoguelikeRacerGame;
use crate::primatives::EntityProperties;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
mod create_inventory_test_items;
pub mod outfit_new_character;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Character {
    pub name_of_controlling_user: String,
    pub entity_properties: EntityProperties,
    pub combatant_properties: CombatantProperties,
    pub unspent_ability_points: u8,
    pub actions_taken: u8,
}

impl Character {
    pub fn new(
        game: &mut RoguelikeRacerGame,
        name: &str,
        combatant_class: CombatantClass,
        name_of_controlling_user: String,
    ) -> Character {
        let mut character = Character {
            name_of_controlling_user: name_of_controlling_user.clone(),
            entity_properties: EntityProperties {
                id: game.id_generator.get_next_entity_id(),
                name: name.to_owned(),
            },
            combatant_properties: CombatantProperties::new(
                &combatant_class,
                &CombatantSpecies::Humanoid,
                HashMap::new(),
                CombatantControlledBy::Player(name_of_controlling_user),
            ),
            unspent_ability_points: 1,
            actions_taken: 0,
        };

        character.combatant_properties.abilities.insert(
            CombatantAbilityNames::Attack,
            CombatantAbility::create_by_name(&CombatantAbilityNames::Attack),
        );
        character.combatant_properties.abilities.insert(
            CombatantAbilityNames::AttackMeleeMainhand,
            CombatantAbility::create_by_name(&CombatantAbilityNames::AttackMeleeMainhand),
        );
        character.combatant_properties.abilities.insert(
            CombatantAbilityNames::AttackMeleeOffhand,
            CombatantAbility::create_by_name(&CombatantAbilityNames::AttackMeleeOffhand),
        );
        character.combatant_properties.abilities.insert(
            CombatantAbilityNames::AttackRangedMainhand,
            CombatantAbility::create_by_name(&CombatantAbilityNames::AttackRangedMainhand),
        );

        outfit_new_character(game, &mut character);

        character
    }
}
