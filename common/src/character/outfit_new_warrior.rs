use super::Character;
use crate::{
    combatants::{
        abilities::{CombatantAbility, CombatantAbilityNames},
        CombatAttributes,
    },
    game::RoguelikeRacerGame,
    items::{
        equipment::{
            one_handed_melee_weapons::OneHandedMeleeWeapons,
            weapon_properties::{DamageClassifications, DamageTypes, WeaponProperties},
            EquipmentProperties, EquipmentSlots, EquipmentTypes,
        },
        Item, ItemProperties,
    },
    primatives::{EntityProperties, MaxAndCurrent, Range},
};
use std::collections::HashMap;

pub fn outfit_new_warrior(game: &mut RoguelikeRacerGame, character: &mut Character) {
    let combatant_properties = &mut character.combatant_properties;
    let inherent_attributes = &mut combatant_properties.inherent_attributes;
    inherent_attributes.insert(CombatAttributes::Damage, 1);
    inherent_attributes.insert(CombatAttributes::Strength, 2);
    inherent_attributes.insert(CombatAttributes::Dexterity, 1);
    inherent_attributes.insert(CombatAttributes::Vitality, 2);
    inherent_attributes.insert(CombatAttributes::Resilience, 2);

    combatant_properties.abilities.insert(
        CombatantAbilityNames::ArmorBreak,
        CombatantAbility::new(&CombatantAbilityNames::ArmorBreak),
    );
    combatant_properties.abilities.insert(
        CombatantAbilityNames::Heal,
        CombatantAbility::new(&CombatantAbilityNames::Heal),
    );

    let entity_properties = EntityProperties {
        id: game.id_generator.get_next_entity_id(),
        name: String::from("Pointy Stick"),
    };

    let equipment_properties = EquipmentProperties {
        equipment_type: EquipmentTypes::OneHandedMeleeWeapon(
            OneHandedMeleeWeapons::Stick,
            WeaponProperties {
                damage_classifications: vec![DamageClassifications::Physical(
                    DamageTypes::Piercing,
                )],
                damage: Range::new(1, 4),
            },
        ),
        durability: Some(MaxAndCurrent::new(15, 15)),
        attributes: HashMap::new(),
        affixes: vec![],
        requirements: HashMap::new(),
        traits: None,
    };

    let starting_weapon = Item {
        entity_properties,
        item_level: 1,
        item_properties: ItemProperties::Equipment(equipment_properties),
    };
    for _ in 0..10 {
        let random_equipment = Item::generate(&mut game.id_generator, 1);
        character.inventory.items.push(random_equipment);
    }

    combatant_properties
        .equipment
        .insert(EquipmentSlots::RightHand, starting_weapon);
}
