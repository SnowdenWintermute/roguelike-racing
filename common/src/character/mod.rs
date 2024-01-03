use self::inventory::CharacterInventory;
use self::outfit_new_warrior::outfit_new_warrior;
use crate::combatants::abilities::CombatantAbility;
use crate::combatants::abilities::CombatantAbilityNames;
use crate::combatants::CombatantClass;
use crate::combatants::CombatantControlledBy;
use crate::combatants::CombatantProperties;
use crate::game::RoguelikeRacerGame;
use crate::primatives::EntityProperties;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
mod equip_item;
pub mod inventory;
pub mod outfit_new_warrior;
mod unequip_item;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Character {
    pub name_of_controlling_user: String,
    pub entity_properties: EntityProperties,
    pub combatant_properties: CombatantProperties,
    pub inventory: CharacterInventory,
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
                HashMap::new(),
                CombatantControlledBy::Player(name_of_controlling_user),
            ),
            inventory: CharacterInventory::new(),
            unspent_ability_points: 1,
            actions_taken: 0,
        };

        character.combatant_properties.abilities.insert(
            CombatantAbilityNames::Attack,
            CombatantAbility::new(CombatantAbilityNames::Attack),
        );

        match combatant_class {
            CombatantClass::Mage => {}
            CombatantClass::Rogue => {}
            CombatantClass::Warrior => outfit_new_warrior(game, &mut character),
            CombatantClass::None => {}
        }

        character
    }
}
